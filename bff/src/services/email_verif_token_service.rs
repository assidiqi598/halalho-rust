use bson::{doc, oid::ObjectId};
use chrono::Utc;
use mongodb::{
    Database,
    error::{ErrorKind, WriteFailure},
};

use crate::{
    models::email_verif_token::{NewEmailVerifToken, EMAIL_VERIF_TOKENS_COLL, EmailVerifToken},
    types::error::CustomError,
};

pub struct VerifEmailTokenService {
    db: Database,
}

impl VerifEmailTokenService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_token(&self, data: &NewEmailVerifToken) -> Result<ObjectId, CustomError> {
        match self
            .db
            .collection::<NewEmailVerifToken>(EMAIL_VERIF_TOKENS_COLL)
            .insert_one(data)
            .await
        {
            Ok(v) => {
                tracing::info!("Created verif email token with id: {}", v.inserted_id);
                Ok(v.inserted_id.as_object_id().unwrap())
            }
            Err(error) => {
                tracing::error!("Error creating verif email token: {:?}", error);

                match error.kind.as_ref() {
                    ErrorKind::Write(WriteFailure::WriteError(w)) if w.code == 11000 => {
                        Err(CustomError::DuplicateKey(data.tokenHash.to_owned()))
                    }
                    _ => Err(CustomError::MongoError(error)),
                }
            }
        }
    }

    // pub async fn get_by_token_hash(
    //     &self,
    //     token_hash: &str,
    // ) -> Result<EmailVerifToken, CustomError> {
    //     match self
    //         .db
    //         .collection::<EmailVerifToken>(EMAIL_VERIF_TOKENS_COLL)
    //         .find_one(doc! { "tokenHash": token_hash })
    //         .await
    //     {
    //         Ok(Some(item)) => Ok(item),
    //         Ok(None) => Err(CustomError::NotFoundError(token_hash.to_owned())),
    //         Err(err) => {
    //             tracing::error!("Error finding email verif token {}: {:?}", token_hash, err);
    //             Err(CustomError::MongoError(err))
    //         }
    //     }
    // }

    pub async fn find_valid_token_then_update(
        &self,
        token_hash: &str,
        user_id: &str,
    ) -> Result<(), CustomError> {
        let user_obj_id = ObjectId::parse_str(user_id).map_err(|e| {
            tracing::error!("Error while parsing {}: {:?}", user_id, e);
            CustomError::InvalidIDError(user_id.to_owned())
        })?;

        match self
            .db
            .collection::<EmailVerifToken>(EMAIL_VERIF_TOKENS_COLL)
            .find_one_and_update(
                doc! {
                    "tokenHash": token_hash,
                    "userId": user_obj_id,
                    "usedAt": { "$eq": null },
                    "expiresAt": { "$gt": Utc::now() }
                },
                doc! {
                    "$set": { "usedAt": Utc::now() }
                },
            )
            .await
        {
            Ok(Some(_)) => Ok(()),
            Ok(None) => Err(CustomError::InvalidToken),
            Err(err) => {
                tracing::error!(
                    "Error finding then updating email verif token {}: {:?}",
                    token_hash,
                    err
                );
                Err(CustomError::InvalidToken)
            }
        }
    }
}
