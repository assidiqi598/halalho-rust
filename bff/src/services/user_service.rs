use mongodb::{Database, error::Error};

use crate::{dtos::auth_dto::RegisterReqDto, models::user::User};

pub struct UserService {
    pub db: Database,
}

impl UserService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    pub async fn crate_user(&self, data: &RegisterReqDto) -> Result<Self, Error> {
        
    }
}
