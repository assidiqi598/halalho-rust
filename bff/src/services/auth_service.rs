use crate::{
    dtos::auth_dto::AuthResDto,
    types::{claims::Claims, error::CustomError, keys::KEYS, refresh_claims::RefreshClaims},
    utils::datetime::now_epoch,
};
use argon2::{
    Argon2,
    password_hash::{
        Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};
use bson::uuid;
use jsonwebtoken::{Algorithm, Header, Validation, decode, encode, errors::ErrorKind};
use rand::TryRngCore;
use sha2::{Digest, Sha256};
use std::env::var;

const ACCESS_EXP_MINUTES: u32 = 15 * 60;
pub const REFRESH_EXP_DAYS: u32 = 7 * 24 * 3600;
pub const EMAIL_VERIFICATION_EXP_MINUTES: u32 = 3600;

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn hash_password(&self, password: String) -> Result<String, Error> {
        let password_as_bytes = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2.hash_password(password_as_bytes, &salt)?.to_string();

        Ok(password_hash)
    }

    pub fn verify_password(&self, password: String, password_hash: String) -> Result<(), Error> {
        let password_as_bytes = password.as_bytes();

        let parsed_hash = PasswordHash::new(&password_hash)?;

        Argon2::default().verify_password(password_as_bytes, &parsed_hash)
    }

    pub fn generate_tokens(
        &self,
        user_id: &str,
    ) -> Result<(AuthResDto, String, usize), CustomError> {
        let claims = Claims {
            sub: user_id.to_owned(),
            exp: now_epoch() + ACCESS_EXP_MINUTES as usize,
            aud: var("JWT_AUDIENCE").expect("JWT_AUDIENCE missing"),
            iss: var("JWT_ISSUER").expect("JWT_ISSUER missing"),
        };

        let refresh_claims = RefreshClaims {
            sub: user_id.to_owned(),
            exp: now_epoch() + REFRESH_EXP_DAYS as usize,
            jti: uuid::Uuid::new().to_string(),
            aud: var("JWT_AUDIENCE").expect("JWT_AUDIENCE missing"),
            iss: var("JWT_ISSUER").expect("JWT_ISSUER missing"),
        };

        let access_token = encode(&Header::new(Algorithm::EdDSA), &claims, &KEYS.encoding)
            .map_err(|_| CustomError::TokenCreation)?;

        let refresh_token = encode(
            &Header::new(Algorithm::EdDSA),
            &refresh_claims,
            &KEYS.encoding,
        )
        .map_err(|_| CustomError::TokenCreation)?;

        Ok((
            AuthResDto::new(access_token, refresh_token),
            refresh_claims.jti,
            refresh_claims.exp,
        ))
    }

    pub fn decode_refresh_token(&self, refresh_token: &str) -> Result<RefreshClaims, CustomError> {
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.set_audience(&[var("JWT_AUDIENCE").expect("JWT_AUDIENCE missing")]);
        validation.set_issuer(&[var("JWT_ISSUER").expect("JWT_ISSUER missing")]);

        match decode::<RefreshClaims>(
            refresh_token,
            &KEYS.decoding,
            &validation,
        ) {
            Ok(value) => Ok(value.claims),
            Err(err) => match err.kind() {
                ErrorKind::ExpiredSignature => Err(CustomError::TokenExpired),
                _ => Err(CustomError::InvalidToken),
            },
        }
    }

    pub fn generate_email_verification_token(&self) -> Result<(String, String), CustomError> {
        let mut bytes = [0u8; 32];
        rand::rngs::OsRng
            .try_fill_bytes(&mut bytes)
            .map_err(|_| CustomError::TokenCreation)?;

        let raw_token = hex::encode(bytes);

        let mut hasher = Sha256::new();
        hasher.update(&raw_token);
        let token_hash = hex::encode(hasher.finalize());

        Ok((raw_token, token_hash))
    }
}
