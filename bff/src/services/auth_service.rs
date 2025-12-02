use crate::{
    dtos::auth_dto::AuthResDto,
    types::{claims::Claims, error::CustomError, keys::KEYS, refresh_claims::RefreshClaims},
    utils::now_epoch,
};
use argon2::{
    Argon2,
    password_hash::{
        Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};
use bson::uuid;
use jsonwebtoken::{Header, Validation, decode, encode, errors::ErrorKind};

const ACCESS_EXP_MINUTES: u32 = 15 * 60;
pub const REFRESH_EXP_DAYS: u32 = 7 * 24 * 3600;
pub const EMAIL_VERIFICATION_EXP_MINUTES: u32 = 3600;

pub struct AuthService {}

impl AuthService {
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
        };

        let refresh_claims = RefreshClaims {
            sub: user_id.to_owned(),
            exp: now_epoch() + REFRESH_EXP_DAYS as usize,
            jti: uuid::Uuid::new().to_string(),
        };

        let access_token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| CustomError::TokenCreation)?;

        let refresh_token = encode(&Header::default(), &refresh_claims, &KEYS.encoding)
            .map_err(|_| CustomError::TokenCreation)?;

        Ok((
            AuthResDto::new(access_token, refresh_token),
            refresh_claims.jti,
            refresh_claims.exp,
        ))
    }

    pub fn decode_refresh_token(&self, refresh_token: &str) -> Result<RefreshClaims, CustomError> {
        match decode::<RefreshClaims>(refresh_token, &KEYS.decoding, &Validation::default()) {
            Ok(value) => Ok(value.claims),
            Err(err) => match err.kind() {
                ErrorKind::ExpiredSignature => Err(CustomError::TokenExpired),
                _ => Err(CustomError::InvalidToken),
            },
        }
    }

    pub fn decode_access_token_without_exp(
        &self,
        access_token: &str,
    ) -> Result<Claims, CustomError> {
        let mut validation = Validation::default();
        validation.validate_exp = false;

        match decode::<Claims>(access_token, &KEYS.decoding, &validation) {
            Ok(value) => Ok(value.claims),
            Err(err) => match err.kind() {
                ErrorKind::ExpiredSignature => Err(CustomError::TokenExpired),
                _ => Err(CustomError::InvalidToken),
            },
        }
    }
}
