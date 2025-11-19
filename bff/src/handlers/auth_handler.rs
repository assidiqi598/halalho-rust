use std::sync::Arc;

use axum::{Json, extract::State, debug_handler};
use chrono::Utc;
use crate::{AppState, dtos::{auth_dto, general_res_dto::GeneralResDto}, models::user::NewUser, services::auth_service::hash_password};
use crate::error::CustomError;

#[debug_handler]
pub async fn register(State(state): State<Arc<AppState>>, Json(payload): Json<auth_dto::RegisterReqDto>) -> Result<Json<GeneralResDto>, CustomError> {
    
    if payload.email.is_empty() || !payload.email.contains("@") || payload.username.len() < 5 || payload.password.len() < 8 {
        return Err(CustomError::MissingCredentials);
    }

    let password_hash = match hash_password(payload.password) {
        Ok(value) => value,
        Err(_) => return Err(CustomError::HashError)
    };

    // let password_hash = hash_password(payload.password)
    // .map_err(|_| CustomError::HashError)?;

    let user = NewUser {
        email: payload.email,
        username: payload.username,
        password: password_hash,
        createdAt: Utc::now(),
        updatedAt: Utc::now()
    };
    
    state.user_service.create_user(&user).await?;

    Ok(Json(GeneralResDto {
        message: "Ok".to_string(),
        status_code: 201
    }))
}

pub async fn login(Json(payload): Json<auth_dto::LoginReqDto>) -> Result<Json<GeneralResDto>, CustomError> {
    if payload.email.is_empty() || !payload.email.contains("@") || payload.password.len() < 8 {
        return Err(CustomError::MissingCredentials);
    }

    Ok(Json(GeneralResDto {
        message: "Ok".to_string(),
        status_code: 201
    }))
}

pub async fn logout() {
    // Implement logout
}

