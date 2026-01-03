

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
    #[error("Token expired")]
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
    SendEmailError,
    #[error("AMQP publish error")]
    AMQPPublishError(#[from] amqprs::error::Error),
    #[error("Serialization error")]
    SerializationError,
}
