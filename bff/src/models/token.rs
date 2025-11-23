use chrono::prelude::*;
use mongodb::bson::{oid::ObjectId};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub userId: ObjectId,
    pub token: String,
    pub isRevoked: bool,
    #[serde(
      serialize_with = "bson",
      deserialize_with = "bson::serde_helpers::datetime::FromChrono04DateTime::deserialize"
    )]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewToken {
    pub userId: ObjectId,
    pub token: String,
    pub isRevoked: bool,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}
