use mongodb::{
    Database,
    error::{ErrorKind, WriteFailure},
};
use bson::{doc, oid::ObjectId};

use crate::{
    error::CustomError,
    models::user::{NewUser, User},
};

pub struct UserService {
    pub db: Database,
}

impl UserService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, data: &NewUser) -> Result<ObjectId, CustomError> {
        match self
            .db
            .collection::<NewUser>("users")
            .insert_one(data)
            .await
        {
            Ok(value) => {
                tracing::debug!("Created a user with _id: {}", &value.inserted_id);
                Ok(value.inserted_id.as_object_id().unwrap())
            }
            Err(error) => {
                tracing::debug!("Error inserting document: {}", error);

                match error.kind.as_ref() {
                    ErrorKind::Write(WriteFailure::WriteError(w)) if w.code == 11000 => {
                        Err(CustomError::DuplicateKey(data.email.clone()))
                    }
                    _ => Err(CustomError::MongoError(error)),
                }
            }
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, CustomError> {
        match self
            .db
            .collection::<User>("users")
            .find_one(doc! { "email": email.to_lowercase() })
            .await
        {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(CustomError::NotFoundError(email.to_owned())),
            Err(err) => {
                tracing::debug!("Error finding user: {}", err);
                Err(CustomError::MongoError(err))
            }
        }
    }
}
