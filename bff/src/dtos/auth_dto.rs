use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RegisterReqDto {
    pub email: String,
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct LoginReqDto {
    pub email: String,
    pub password: String
}