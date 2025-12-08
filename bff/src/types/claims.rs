use crate::types::error::CustomError;
use crate::types::keys::KEYS;
use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{Algorithm, Validation, decode, errors::ErrorKind};
use serde::{Deserialize, Serialize};
use std::{env::var, fmt::Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub aud: String,
    pub iss: String,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sub: {}\nExpire: {}", self.sub, self.exp)
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = CustomError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| CustomError::InvalidToken)?;
        // Decode the user data
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.set_audience(&[var("JWT_AUDIENCE").expect("JWT_AUDIENCE missing")]);
        validation.set_issuer(&[var("JWT_ISSUER").expect("JWT_ISSUER missing")]);

        match decode::<Claims>(bearer.token(), &KEYS.decoding, &validation) {
            Ok(value) => {
                tracing::info!("Req from {} has just arrived", value.claims.sub);
                Ok(value.claims)
            }
            Err(err) => match err.kind() {
                ErrorKind::ExpiredSignature => Err(CustomError::TokenExpired),
                _ => Err(CustomError::InvalidToken),
            },
        }
    }
}
