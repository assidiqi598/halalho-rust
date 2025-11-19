use mongodb::{Database, error::{ErrorKind, WriteFailure}};

use crate::{error::CustomError, models::user::NewUser};

pub struct UserService {
    pub db: Database,
}

impl UserService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, data: &NewUser) -> Result<(), CustomError> {
        match self
            .db
            .collection::<NewUser>("users")
            .insert_one(data)
            .await
        {
            Ok(value) => {
                println!("Created a user with _id: {}", value.inserted_id);
                Ok(())
            },
            Err(error) => {
                eprintln!("Error inserting document: {}", error);

                match error.kind.as_ref() {
                    ErrorKind::Write(WriteFailure::WriteError(w)) if w.code == 11000 => {
                        Err(CustomError::DuplicateKey(data.email.clone()))
                    },
                    _ => Err(CustomError::MongoError(error))
                }
            }
        }
    }

    // pub async fn get_user_by_email(&self, email: &'static string) ->
}
