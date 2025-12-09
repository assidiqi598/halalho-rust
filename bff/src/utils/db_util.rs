use bson::doc;
use mongodb::{Database, IndexModel, error::Error, options::IndexOptions};
use std::time::Duration;

use crate::models::{
    refresh_token::{REFRESH_TOKENS_COLL, RefreshToken},
    user::{USERS_COLL, User},
    email_verif_token::{EMAIL_VERIF_TOKENS_COLL, EmailVerifToken},
};

const DATA_REMOVAL_AFTER_SECS: u64 = 30 * 24 * 3600;

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
                    .expire_after(Some(Duration::from_secs(DATA_REMOVAL_AFTER_SECS)))
                    .build(),
            )
            .build(),
    ];
    // insert the indexes for refresh tokens
    refresh_tokens.create_indexes(refresh_token_indexes).await?;

    // do also for email verification tokens
    let email_verfication_tokens = db.collection::<EmailVerifToken>(EMAIL_VERIF_TOKENS_COLL);

    let email_verif_indexes = vec![
        IndexModel::builder()
            .keys(doc! { "tokenHash": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build(),
        IndexModel::builder()
            .keys(doc! { "createdAt": 1 })
            .options(
                IndexOptions::builder()
                    .expire_after(Some(Duration::from_secs(DATA_REMOVAL_AFTER_SECS)))
                    .build(),
            )
            .build(),
        IndexModel::builder()
            .keys(doc! {
                "tokenHash": 1,
                "userId": 1,
            })
            .build(),
    ];

    email_verfication_tokens
        .create_indexes(email_verif_indexes)
        .await?;

    Ok(())
}
