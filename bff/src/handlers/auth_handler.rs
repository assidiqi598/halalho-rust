use crate::dtos::auth_dto::AuthResDto;
use crate::models::refresh_token::NewRefreshToken;
use crate::models::verif_email_token::NewVerifEmailToken;
use crate::services::auth_service::EMAIL_VERIFICATION_EXP_MINUTES;
use crate::services::email_service::EmailTemplateValues;
use crate::types::claims::Claims;
use crate::types::email::Email;
use crate::types::error::CustomError;
use crate::types::verify_email::VerifyEmail;
use crate::utils::datetime::now_epoch;
use crate::{
    AppState,
    dtos::{auth_dto, general_res_dto::GeneralResDto},
    models::user::NewUser,
};
use axum::{Json, debug_handler, extract::State, http::StatusCode};
use chrono::offset::LocalResult;
use chrono::{TimeZone, Utc};
use std::sync::Arc;

#[debug_handler]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<auth_dto::RegisterReqDto>,
) -> Result<Json<AuthResDto>, CustomError> {
    if payload.email.is_empty()
        || !payload.email.contains("@")
        || payload.username.len() < 5
        || payload.password.len() < 8
    {
        return Err(CustomError::MissingCredentials);
    }

    let password_hash = match state.auth_service.hash_password(payload.password) {
        Ok(value) => value,
        Err(_) => return Err(CustomError::HashError),
    };

    // let password_hash = hash_password(payload.password)
    // .map_err(|_| CustomError::HashError)?;

    // Create user in DB
    let user = NewUser {
        email: payload.email.to_lowercase(),
        username: payload.username,
        password: password_hash,
        isEmailVerified: false,
        lastLoginAt: Utc::now(),
        createdAt: Utc::now(),
        updatedAt: Utc::now(),
    };

    let user_id = state.user_service.create_user(&user).await?;

    // Send email to verify email address
    tokio::spawn({
        let state = state.clone();
        let username = user.username.clone();
        let email = user.email.clone();
        let user_id = user_id.clone();

        async move {
            if let Err(err) = async {
                let (object_bytes, ext) = state
                    .storage_service
                    .get_object("halalho/email-templates/verify-email.html")
                    .await
                    .map_err(|_| CustomError::R2Error)?;

                let object_extension = ext.ok_or(CustomError::R2Error)?;

                let (raw_token, token_hash) =
                    state.auth_service.generate_email_verification_token()?;

                let new_verif_email_token = NewVerifEmailToken {
                    userId: user_id,
                    tokenHash: token_hash,
                    expiresAt: Utc
                        .timestamp_opt(
                            (now_epoch() + EMAIL_VERIFICATION_EXP_MINUTES as usize) as i64,
                            0,
                        )
                        .single()
                        .ok_or_else(|| {
                            tracing::error!(
                                "Error converting timestamp for verif email token expiration"
                            );
                            CustomError::TokenCreation
                        })?,
                    createdAt: Utc::now(),
                };

                state.verif_email_token_service.create_token(&new_verif_email_token).await?;

                let values =
                    EmailTemplateValues::VerifyEmailValues(VerifyEmail::new(&username, &raw_token));

                let email_html = state.email_service.prepare_template(
                    &object_bytes,
                    &object_extension,
                    values,
                )?;

                let email: Email = Email::new(
                    vec![(&username, &email)],
                    email_html,
                    "Please verify your email-address",
                );

                state.email_service.send_transactional_email(email).await?;

                Ok::<(), CustomError>(())
            }
            .await
            {
                tracing::error!("Failed to send verification email for {}: {:?}", email, err)
            }
        }
    });

    // Generate tokens for authentication
    let (tokens, jti, exp) = state
        .auth_service
        .generate_tokens(&user_id.to_hex())
        .map_err(|_| CustomError::TokenCreation)?;

    let expires_at = match Utc.timestamp_opt(exp as i64, 0) {
        LocalResult::Single(dt) => dt,
        _ => {
            tracing::error!("Error converting timestamp");
            return Err(CustomError::TokenCreation);
        }
    };

    // let expiresAt = Utc.timestamp_opt(exp as i64, 0).single().ok_or_else(|| {
    //     tracing::error!("Error converting timestamp");
    //     CustomError::TokenCreation
    // })?;

    let new_refresh_token = NewRefreshToken {
        userId: user_id,
        token: jti,
        isRevoked: false,
        createdAt: Utc::now(),
        expiresAt: expires_at,
        usedAt: None,
    };

    state
        .refresh_token_service
        .create_token(&new_refresh_token)
        .await?;

    tracing::info!("User {} has logged in after registration", user_id.to_hex());

    Ok(Json(tokens))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<auth_dto::LoginReqDto>,
) -> Result<Json<AuthResDto>, CustomError> {
    if payload.email.is_empty() || !payload.email.contains("@") || payload.password.len() < 8 {
        return Err(CustomError::MissingCredentials);
    }

    let user = state.user_service.get_user_by_email(&payload.email).await?;

    match state
        .auth_service
        .verify_password(payload.password, user.password)
    {
        Ok(_) => {
            tracing::info!("User {} has logged in", user.email);

            let (tokens, jti, exp) = state
                .auth_service
                .generate_tokens(&user.id.to_string())
                .map_err(|_| CustomError::TokenCreation)?;

            let expires_at = match Utc.timestamp_opt(exp as i64, 0) {
                LocalResult::Single(dt) => dt,
                _ => {
                    tracing::error!("Error converting timestamp");
                    return Err(CustomError::TokenCreation);
                }
            };

            let new_refresh_token = NewRefreshToken {
                userId: user.id,
                token: jti,
                isRevoked: false,
                createdAt: Utc::now(),
                expiresAt: expires_at,
                usedAt: None,
            };

            state
                .refresh_token_service
                .create_token(&new_refresh_token)
                .await?;

            Ok(Json(tokens))
        }
        Err(_) => Err(CustomError::WrongCredentials),
    }
}

pub async fn logout(
    claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<auth_dto::LogoutDto>,
) -> Result<Json<GeneralResDto>, CustomError> {
    if payload.refresh_token.is_empty() {
        tracing::debug!("No refresh token provided");
        return Err(CustomError::MissingCredentials);
    }

    let refresh_claims = state
        .auth_service
        .decode_refresh_token(&payload.refresh_token)?;

    if refresh_claims.jti.is_empty() {
        tracing::debug!("Missing jti");
        return Err(CustomError::MissingCredentials);
    }

    if refresh_claims.sub != claims.sub {
        return Err(CustomError::WrongCredentials);
    }

    state
        .refresh_token_service
        .revoke_token(&refresh_claims.jti)
        .await?;

    Ok(Json(GeneralResDto {
        message: "Ok".to_string(),
        status_code: StatusCode::OK.as_u16(),
    }))
}

pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<auth_dto::LogoutDto>,
) -> Result<Json<AuthResDto>, CustomError> {
    if payload.refresh_token.is_empty() {
        tracing::debug!("Missing refresh token");
        return Err(CustomError::MissingCredentials);
    }

    let current_refresh_claims = state
        .auth_service
        .decode_refresh_token(&payload.refresh_token)?;

    if current_refresh_claims.jti.is_empty() {
        tracing::debug!("Missing jti");
        return Err(CustomError::MissingCredentials);
    }

    let current_token = state
        .refresh_token_service
        .get_token_by_jti(&current_refresh_claims.jti)
        .await?;

    if current_token.expiresAt < Utc::now() {
        return Err(CustomError::TokenExpired);
    }

    let mut should_revoke = true;

    if current_token.isRevoked {
        match current_token.usedAt {
            Some(used_at) if (Utc::now() - used_at).num_seconds() > 90 => {
                return Err(CustomError::TokenExpired);
            }
            Some(used_at) if (Utc::now() - used_at).num_seconds() < 90 => {
                // don't revoke again
                should_revoke = false;
            }
            None => return Err(CustomError::InvalidToken),
            _ => return Err(CustomError::InvalidToken),
        }
    }

    if should_revoke {
        state
            .refresh_token_service
            .revoke_token(&current_refresh_claims.jti)
            .await?;
    }

    let user = state
        .user_service
        .get_user_by_id(&current_refresh_claims.sub)
        .await?;

    let (tokens, jti, exp) = state
        .auth_service
        .generate_tokens(&user.id.to_hex())
        .map_err(|_| CustomError::TokenCreation)?;

    let expires_at = match Utc.timestamp_opt(exp as i64, 0) {
        LocalResult::Single(dt) => dt,
        _ => {
            tracing::error!("Error converting timestamp");
            return Err(CustomError::TokenCreation);
        }
    };

    let new_refresh_token = NewRefreshToken {
        userId: user.id,
        token: jti,
        isRevoked: false,
        createdAt: Utc::now(),
        expiresAt: expires_at,
        usedAt: None,
    };

    state
        .refresh_token_service
        .create_token(&new_refresh_token)
        .await?;

    Ok(Json(tokens))
}

pub async fn verify_email(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<auth_dto::VerifyEmailDto>,
) -> Result<Json<GeneralResDto>, CustomError> {
    Ok(Json(GeneralResDto {
        status_code: 200,
        message: "Ok".to_owned(),
    }))
}

// TODO: implement password reset
