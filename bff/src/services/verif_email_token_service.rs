use bson::oid::ObjectId;
use mongodb::{Database, error::{ErrorKind, WriteFailure}};

use crate::{
    models::verif_email_token::{NewVerifEmailToken, VERIF_EMAIL_TOKENS_COLL},
    types::error::CustomError,
};

pub struct VerifEmailTokenService {
    db: Database,
}

impl VerifEmailTokenService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_token(&self, data: &NewVerifEmailToken) -> Result<ObjectId, CustomError> {
        match self
            .db
            .collection::<NewVerifEmailToken>(VERIF_EMAIL_TOKENS_COLL)
            .insert_one(data)
            .await {
                Ok(v) => {
                    tracing::info!("Created verif email token with id: {}", v.inserted_id);
                    Ok(v.inserted_id.as_object_id().unwrap())
                },
                Err(error) => {
                    tracing::error!("Error creating verif email token: {:?}", error);

                    match error.kind.as_ref() {
                        ErrorKind::Write(WriteFailure::WriteError(w)) if w.code == 11000 => {
                            Err(CustomError::DuplicateKey(data.tokenHash.to_owned()))
                        },
                        _ => Err(CustomError::MongoError(error))
                    }
                }
            }
    }
}
