use crate::{
    models::refresh_token::{NewRefreshToken, REFRESH_TOKENS_COLL, RefreshToken}, types::error::CustomError
};
use chrono::Utc;
use mongodb::{
    Database,
    bson::doc,
    error::{ErrorKind, WriteFailure},
};

pub struct RefreshTokenService {
    db: Database,
}

impl RefreshTokenService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get_token_by_jti(&self, data: &str) -> Result<RefreshToken, CustomError> {
        match self
            .db
            .collection::<RefreshToken>(REFRESH_TOKENS_COLL)
            .find_one(doc! { "token": data })
            .await
        {
            Ok(Some(token)) => Ok(token),
            Ok(None) => Err(CustomError::NotFoundError(data.to_owned())),
            Err(err) => {
                tracing::debug!("Error finding token: {}", err);
                Err(CustomError::MongoError(err))
            }
        }
    }

    pub async fn create_token(&self, data: &NewRefreshToken) -> Result<(), CustomError> {
        match self
            .db
            .collection::<NewRefreshToken>(REFRESH_TOKENS_COLL)
            .insert_one(data)
            .await
        {
            Ok(value) => {
                tracing::debug!("Created a token with _id: {}", value.inserted_id);
                Ok(())
            }
            Err(error) => {
                tracing::debug!("Error inserting document: {}", error);

                match error.kind.as_ref() {
                    ErrorKind::Write(WriteFailure::WriteError(w)) if w.code == 11000 => {
                        Err(CustomError::DuplicateKey(data.token.clone()))
                    }
                    _ => Err(CustomError::MongoError(error)),
                }
            }
        }
    }

    pub async fn revoke_token(&self, token: &str) -> Result<(), CustomError> {
        match self
            .db
            .collection::<RefreshToken>(REFRESH_TOKENS_COLL)
            .update_one(
                doc! { "token": token, "isRevoked": false },
                doc! { "$set": doc! { "isRevoked": true, "usedAt": Utc::now() } },
            )
            .await
        {
            Ok(value) => {
                tracing::debug!("{:#?} has been revoked", value.modified_count);
                Ok(())
            }
            Err(error) => {
                tracing::debug!("Error revoking token: {}", error);
                Err(CustomError::MongoError(error))
            }
        }
    }
}
