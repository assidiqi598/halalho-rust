use bson::{doc, oid::ObjectId};
use mongodb::{
    Database,
    error::{ErrorKind, WriteFailure},
};

use crate::{
    models::user::{NewUser, USERS_COLL, User}, types::error::CustomError
};

pub struct UserService {
    db: Database,
}

impl UserService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, data: &NewUser) -> Result<ObjectId, CustomError> {
        match self
            .db
            .collection::<NewUser>(USERS_COLL)
            .insert_one(data)
            .await
        {
            Ok(value) => {
                tracing::info!("Created a user with _id: {}", &value.inserted_id);
                Ok(value.inserted_id.as_object_id().unwrap())
            }
            Err(error) => {
                tracing::error!("Error inserting document: {:?}", error);

                match error.kind.as_ref() {
                    ErrorKind::Write(WriteFailure::WriteError(w)) if w.code == 11000 => {
                        Err(CustomError::DuplicateKey(data.email.to_owned()))
                    }
                    _ => Err(CustomError::MongoError(error)),
                }
            }
        }
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<User, CustomError> {
        let user_id =
            ObjectId::parse_str(id).map_err(|_| CustomError::InvalidIDError(id.to_owned()))?;

        match self
            .db
            .collection::<User>(USERS_COLL)
            .find_one(doc! { "_id": user_id })
            .await
        {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(CustomError::NotFoundError(id.to_owned())),
            Err(err) => {
                tracing::error!("Error finding user: {:?}", err);
                Err(CustomError::MongoError(err))
            }
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, CustomError> {
        match self
            .db
            .collection::<User>(USERS_COLL)
            .find_one(doc! { "email": email.to_lowercase() })
            .await
        {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(CustomError::NotFoundError(email.to_owned())),
            Err(err) => {
                tracing::debug!("Error finding user: {:?}", err);
                Err(CustomError::MongoError(err))
            }
        }
    }
}
