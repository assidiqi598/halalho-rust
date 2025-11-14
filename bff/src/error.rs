
#[derive(thiserror::Error, Debug)]
pub enum CustomError {
    #[error("MongoDB error")]
    MongoError(#[from] mongodb::error::Error),
    #[error("Duplicate key error: {0}")]
    MongoErrorKind(mongodb::error::ErrorKind),
    #[error("Duplicate key error: {0}")]
    MongoDuplicateError(mongodb::error::Error),
    #[error("Error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("Error serializing BSON")]
    MongoSerializeBsonError(#[from] mongodb::bson::ser::Error),
    #[error("Validation error")]
    MongoDataError(#[from] mongodb::bson::document::ValueAccessError),
    #[error("Invalid ID: {0}")]
    InvalidIDError(String),
    #[error("Note with ID: {0} not found")]
    NotFoundError(String),
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Error during token creation")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
}

