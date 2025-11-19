use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterReqDto {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginReqDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResDto {
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: String,
}

impl AuthResDto {
    fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
            refresh_token,
        }
    }
}
