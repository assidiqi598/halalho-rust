use bson::doc;
use mongodb::{Database, IndexModel, error::Error, options::IndexOptions};
use std::time::Duration;

use crate::{
    models::{
        refresh_token::{REFRESH_TOKENS_COLL, RefreshToken},
        user::{USERS_COLL, User},
    },
    services::auth_service::REFRESH_EXP_DAYS,
};

pub async fn ensure_indexes(db: &Database) -> Result<(), Error> {
    let users = db.collection::<User>(USERS_COLL);

    let user_indexes = vec![
        IndexModel::builder()
            .keys(doc! { "username": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build(),
        IndexModel::builder()
            .keys(doc! { "email": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build(),
    ];

    users.create_indexes(user_indexes).await?;

    let refresh_tokens = db.collection::<RefreshToken>(REFRESH_TOKENS_COLL);

    // indexes for refresh tokens
    let refresh_token_indexes = vec![
        IndexModel::builder()
            .keys(doc! { "token": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build(),
        IndexModel::builder()
            .keys(doc! { "createdAt": 1})
            .options(
                IndexOptions::builder()
                    .expire_after(Some(Duration::from_secs(REFRESH_EXP_DAYS as u64)))
                    .build(),
            )
            .build(),
    ];
    // insert the indexes for refresh tokens
    refresh_tokens.create_indexes(refresh_token_indexes).await?;

    // do also for email verification tokens

    Ok(())
}
