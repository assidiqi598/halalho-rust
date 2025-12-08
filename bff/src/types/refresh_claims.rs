use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: usize,
    pub jti: String,
    pub aud: String,
    pub iss: String
}