use argon2::{
    Argon2,
    password_hash::{
        Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::{env, fmt::Display, sync::LazyLock};

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static Keys: LazyLock<Keys> = LazyLock::new(|| {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub struct Claims {
    sub: String,
    exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sub: {}\nExpire: {}", self.sub, self.exp)
    }   
}

pub fn hash_password(password: String) -> Result<String, Error> {
    let password_as_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password_as_bytes, &salt)?.to_string();

    Ok(password_hash)
}

pub fn verify_password(password: String, password_hash: String) -> Result<(), Error> {
    let password_as_bytes = password.as_bytes();

    let parsed_hash = PasswordHash::new(&password_hash)?;

    Argon2::default().verify_password(password_as_bytes, &parsed_hash)
}
