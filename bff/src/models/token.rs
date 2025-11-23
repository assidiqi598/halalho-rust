use bson::{oid::ObjectId, serde_helpers::datetime::FromChrono04DateTime};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[allow(non_snake_case)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub userId: ObjectId,
    pub token: String,
    pub isRevoked: bool,
    #[serde_as(as = "FromChrono04DateTime")]
    pub createdAt: DateTime<Utc>,
    #[serde_as(as = "FromChrono04DateTime")]
    pub updatedAt: DateTime<Utc>,
}

#[allow(non_snake_case)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewToken {
    pub userId: ObjectId,
    pub token: String,
    pub isRevoked: bool,
    #[serde_as(as = "FromChrono04DateTime")]
    pub createdAt: DateTime<Utc>,
    #[serde_as(as = "FromChrono04DateTime")]
    pub updatedAt: DateTime<Utc>,
}
