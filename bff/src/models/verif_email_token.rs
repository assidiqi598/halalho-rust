use bson::{oid::ObjectId, serde_helpers::datetime::FromChrono04DateTime};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub const VERIF_EMAIL_TOKENS_COLL: &str = "verif_email_tokens";

#[allow(non_snake_case)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifEmailToken {
    #[serde(rename="_id")]
    pub id: ObjectId,
    pub userId: ObjectId,
    pub tokenHash: String,

    #[serde_as(as = "FromChrono04DateTime")]
    pub expiresAt: DateTime<Utc>,

    #[serde_as(as = "FromChrono04DateTime")]
    pub createdAt: DateTime<Utc>
}

#[allow(non_snake_case)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewVerifEmailToken {
    pub userId: ObjectId,
    pub tokenHash: String,

    #[serde_as(as = "FromChrono04DateTime")]
    pub expiresAt: DateTime<Utc>,

    #[serde_as(as = "FromChrono04DateTime")]
    pub createdAt: DateTime<Utc>
}