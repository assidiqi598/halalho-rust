use mongodb::{Database, error::Result};

use crate::{models::user::NewUser};

pub struct UserService {
    pub db: Database,
}

impl UserService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn crate_user(&self, data: &NewUser) -> Result<()> {
        let res = self
            .db
            .collection::<NewUser>("users")
            .insert_one(data)
            .await?;
        println!("Inserted a document with _id: {}", res.inserted_id);

        Ok(())
    }

    // pub async fn get_user_by_email(&self, email: &'static string) ->
}
