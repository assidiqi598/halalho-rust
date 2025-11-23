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
use axum::http::StatusCode;
use axum::{Json, debug_handler, extract::State};
use bson::oid::ObjectId;
use chrono::Utc;

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

    let tokens = state
        .auth_service
        .generate_tokens(&user_id.to_hex())
        .map_err(|_| CustomError::TokenCreation)?;

    let new_refresh_token = NewToken {
        userId: ObjectId::parse_str(&user_id.to_hex()).map_err(|_| CustomError::InvalidIDError(user_id.to_hex()))?,
        token: tokens.refresh_token.clone(),
        isRevoked: false,
        createdAt: Utc::now(),
        updatedAt: Utc::now(),
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

            let tokens = state
                .auth_service
                .generate_tokens(&user.id.to_string())
                .map_err(|_| CustomError::TokenCreation)?;

            let new_refresh_token = NewToken {
                userId: user.id,
                token: tokens.refresh_token.clone(),
                isRevoked: false,
                createdAt: Utc::now(),
                updatedAt: Utc::now(),
            };

            state.token_service.create_token(&new_refresh_token).await?;

            Ok(Json(tokens))
        }
        Err(_) => Err(CustomError::WrongCredentials),
    }
}

pub async fn logout(
    _: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<auth_dto::LogoutDto>,
) -> Result<Json<GeneralResDto>, CustomError> {
    state
        .token_service
        .revoke_token(payload.refresh_token)
        .await?;

    Ok(Json(GeneralResDto {
        message: "Ok".to_string(),
        status_code: StatusCode::CREATED.as_u16(),
    }))
}
