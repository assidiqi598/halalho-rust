use axum::{Json, debug_handler};
use crate::dtos::{auth_dto, general_res_dto::{AuthError, GeneralResDto}};

#[debug_handler]
pub async fn register(Json(payload): Json<auth_dto::RegisterReqDto>) -> Result<Json<GeneralResDto>, AuthError> {
    
    if payload.email.is_empty() || !payload.email.contains("@") || payload.username.len() < 5 || payload.password.len() < 8 {
        return Err(AuthError::MissingCredentials);
    }

    Ok(Json(GeneralResDto {
        message: "Ok".to_string(),
        status_code: 201
    }))
}

pub async fn login(Json(payload): Json<auth_dto::LoginReqDto>) -> Result<Json<GeneralResDto>, AuthError> {
    if payload.email.is_empty() || !payload.email.contains("@") || payload.password.len() < 8 {
        return Err(AuthError::MissingCredentials);
    }

    Ok(Json(GeneralResDto {
        message: "Ok".to_string(),
        status_code: 201
    }))
}

pub async fn logout() {
    // Implement logout
}

