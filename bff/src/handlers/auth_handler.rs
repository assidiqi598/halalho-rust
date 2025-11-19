use std::sync::Arc;

use crate::error::CustomError;
use crate::{
    AppState,
    dtos::{auth_dto, general_res_dto::GeneralResDto},
    models::user::NewUser,
    services::auth_service::{hash_password, verify_password},
};
use axum::{Json, debug_handler, extract::State};
use chrono::Utc;

#[debug_handler]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<auth_dto::RegisterReqDto>,
) -> Result<Json<GeneralResDto>, CustomError> {
    if payload.email.is_empty()
        || !payload.email.contains("@")
        || payload.username.len() < 5
        || payload.password.len() < 8
    {
        return Err(CustomError::MissingCredentials);
    }

    let password_hash = match hash_password(payload.password) {
        Ok(value) => value,
        Err(_) => return Err(CustomError::HashError),
    };

    // let password_hash = hash_password(payload.password)
    // .map_err(|_| CustomError::HashError)?;

    let user = NewUser {
        email: payload.email.to_lowercase(),
        username: payload.username,
        password: password_hash,
        createdAt: Utc::now(),
        updatedAt: Utc::now(),
    };

    state.user_service.create_user(&user).await?;

    Ok(Json(GeneralResDto {
        message: "Ok".to_string(),
        status_code: 201,
    }))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<auth_dto::LoginReqDto>,
) -> Result<Json<GeneralResDto>, CustomError> {
    if payload.email.is_empty() || !payload.email.contains("@") || payload.password.len() < 8 {
        return Err(CustomError::MissingCredentials);
    }

    let user = state.user_service.get_user_by_email(&payload.email).await?;

    match verify_password(payload.password, user.password) {
        Ok(_) => Ok(Json(GeneralResDto {
            message: "Ok".to_string(),
            status_code: 201,
        })),
        Err(_) => Err(CustomError::WrongCredentials)
    }
}

pub async fn logout() {
    // Implement logout
}
