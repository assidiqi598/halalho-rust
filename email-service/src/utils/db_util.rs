use bson::doc;
use mongodb::{Database, IndexModel, error::Error, options::IndexOptions};
use std::time::Duration;

use crate::models::{
    email_verif_token::{EMAIL_VERIF_TOKENS_COLL, EmailVerifToken},
};

const DATA_REMOVAL_AFTER_SECS: u64 = 30 * 24 * 3600;

pub async fn ensure_indexes(db: &Database) -> Result<(), Error> {
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
