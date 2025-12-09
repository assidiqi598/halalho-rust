use bson::{oid::ObjectId, serde_helpers::datetime::FromChrono04DateTime};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub const EMAIL_VERIF_TOKENS_COLL: &str = "email_verif_tokens";

#[allow(non_snake_case)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailVerifToken {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub userId: ObjectId,
    pub tokenHash: String,

    #[serde_as(as = "FromChrono04DateTime")]
    pub expiresAt: DateTime<Utc>,

    #[serde_as(as = "FromChrono04DateTime")]
    pub createdAt: DateTime<Utc>,

    #[serde_as(as = "Option<FromChrono04DateTime>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usedAt: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewEmailVerifToken {
    pub userId: ObjectId,
    pub tokenHash: String,

    #[serde_as(as = "FromChrono04DateTime")]
    pub expiresAt: DateTime<Utc>,

    #[serde_as(as = "FromChrono04DateTime")]
    pub createdAt: DateTime<Utc>,

    #[serde_as(as = "Option<FromChrono04DateTime>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usedAt: Option<DateTime<Utc>>,
}
