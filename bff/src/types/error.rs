use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum CustomError {
    #[error("MongoDB error")]
    MongoError(#[from] mongodb::error::Error),
    #[error("Duplicate key error: {0}")]
    DuplicateKey(String),
    #[error("Invalid ID: {0}")]
    InvalidIDError(String),
    #[error("Not found: {0}")]
    NotFoundError(String),
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Error during token creation")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
    #[error("RefreshToken expired")]
    TokenExpired,
    #[error("Hash error")]
    HashError,
    #[error("Email Template Error")]
    EmailTemplateError,
    #[error("R2 error")]
    R2Error,
    #[error("Reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Error sending email")]
    SendEmailError
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            CustomError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, "Wrong credentials".to_string())
            }
            CustomError::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Missing credentials".to_string())
            }
            CustomError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "RefreshToken creation error".to_string(),
            ),
            CustomError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            CustomError::TokenExpired => (StatusCode::UNAUTHORIZED, "RefreshToken is expired".to_string()),
            CustomError::MongoError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "MongoDB error".to_string(),
            ),
            CustomError::DuplicateKey(key) => {
                (StatusCode::CONFLICT, format!("{} already exists", key))
            }
            CustomError::InvalidIDError(id) => {
                (StatusCode::BAD_REQUEST, format!("Id {id} is invalid"))
            }
            CustomError::NotFoundError(param) => (
                StatusCode::NOT_FOUND,
                format!("Doc with {} not found", param),
            ),
            CustomError::HashError => {
                (StatusCode::NOT_ACCEPTABLE, "Error when hashing".to_string())
            }
            CustomError::EmailTemplateError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error when preparing email template".to_string(),
            ),
            CustomError::R2Error => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Object storage error".to_owned(),
            ),
            CustomError::ReqwestError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error sending request".to_owned()
            ),
            CustomError::SendEmailError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error sending email".to_owned()
            )
        };

        let body = Json(json!({
            "error": err_msg
        }));

        (status, body).into_response()
    }
}
