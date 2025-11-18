use std::sync::Arc;

use axum::{Json, extract::State, debug_handler};
use chrono::Utc;
use crate::{AppState, dtos::{auth_dto, general_res_dto::GeneralResDto}, models::user::NewUser};
use crate::error::CustomError;

#[debug_handler]
pub async fn register(State(state): State<Arc<AppState>>, Json(payload): Json<auth_dto::RegisterReqDto>) -> Result<Json<GeneralResDto>, CustomError> {
    
    if payload.email.is_empty() || !payload.email.contains("@") || payload.username.len() < 5 || payload.password.len() < 8 {
        return Err(CustomError::MissingCredentials);
    }

    let user = NewUser {
        email: payload.email,
        username: payload.username,
        password: payload.password,
        createdAt: Utc::now(),
        updatedAt: Utc::now()
    };
    
    state.user_service.crate_user(&user).await?;

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

