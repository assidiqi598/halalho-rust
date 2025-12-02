use bson::{oid::ObjectId, serde_helpers::datetime::FromChrono04DateTime};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub const USERS_COLL: &str = "users";

#[allow(non_snake_case)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
    pub isEmailVerified: bool,
    #[serde_as(as = "FromChrono04DateTime")]
    pub lastLoginAt: DateTime<Utc>,
    #[serde_as(as = "FromChrono04DateTime")]
    pub createdAt: DateTime<Utc>,
    #[serde_as(as = "FromChrono04DateTime")]
    pub updatedAt: DateTime<Utc>,
}

#[allow(non_snake_case)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub isEmailVerified: bool,
    #[serde_as(as = "FromChrono04DateTime")]
    pub lastLoginAt: DateTime<Utc>,
    #[serde_as(as = "FromChrono04DateTime")]
    pub createdAt: DateTime<Utc>,
    #[serde_as(as = "FromChrono04DateTime")]
    pub updatedAt: DateTime<Utc>,
}
