use std::sync::Arc;

use crate::dtos::auth_dto::AuthResDto;
use crate::error::CustomError;
use crate::models::token::NewToken;
use crate::services::auth_service::Claims;
use crate::{
    AppState,
    dtos::{auth_dto, general_res_dto::GeneralResDto},
    models::user::NewUser,
};
use axum::{Json, debug_handler, extract::State, http::StatusCode};
use chrono::offset::LocalResult;
use chrono::{TimeZone, Utc};

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

    let new_refresh_token = NewToken {
        userId: user_id,
        token: jti,
        isRevoked: false,
        createdAt: Utc::now(),
        expiresAt: expires_at,
        usedAt: None,
    };

    state.token_service.create_token(&new_refresh_token).await?;

    tracing::info!("User {} has logged in after registration", user.email);

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

            let new_refresh_token = NewToken {
                userId: user.id,
                token: jti,
                isRevoked: false,
                createdAt: Utc::now(),
                expiresAt: expires_at,
                usedAt: None,
            };

            state.token_service.create_token(&new_refresh_token).await?;

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
        .token_service
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
        .token_service
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
            .token_service
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

    let new_refresh_token = NewToken {
        userId: user.id,
        token: jti,
        isRevoked: false,
        createdAt: Utc::now(),
        expiresAt: expires_at,
        usedAt: None,
    };

    state.token_service.create_token(&new_refresh_token).await?;

    Ok(Json(tokens))
}

// TODO: implement password reset
