use bson::doc;
use mongodb::{Database, IndexModel, options::IndexOptions};

use crate::{models::{refresh_token::{REFRESH_TOKEN_COLL, RefreshToken}, user::{USERS_COLL, User}}, types::error::CustomError};

pub async fn ensure_indexs(db: &Database) -> Result<(), CustomError> {
    let users = db.collection::<User>(USERS_COLL);

    let user_indexes = vec![
        IndexModel::builder()
            .keys(doc! { "username": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build(),
        IndexModel::builder()
            .keys(doc! { "email": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build()
    ];

    users.create_indexes(user_indexes).await.map_err(|err| {
        tracing::error!("Error during creating user indexes: {:?}", err);
        return CustomError::MongoError(err);
    })?;

    let refresh_tokens = db.collection::<RefreshToken>(REFRESH_TOKEN_COLL);

    // indexes for refresh tokens
    // insert the indexes for refresh tokens

    // do also for email verification tokens

    Ok(())
}
