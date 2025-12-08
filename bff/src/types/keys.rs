use base64::{Engine, engine::general_purpose};
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::{env::var, sync::LazyLock};

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(private: Vec<u8>, public: Vec<u8>) -> Self {
        Self {
            encoding: EncodingKey::from_ed_pem(&private).expect("Invalid private Ed25519 PEM"),
            decoding: DecodingKey::from_ed_pem(&public).expect("Invalid public Ed25519 PEM"),
        }
    }
}

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    // let secret = var("JWT_SECRET").expect("JWT_SECRET must be set");
    let private_b64 = var("JWT_PRIVATE_KEY").expect("JWT_PRIVATE_KEY missing");
    let public_b64 = var("JWT_PUBLIC_KEY").expect("JWT_PUBLIC_KEY missing");

    let private_pem = general_purpose::STANDARD.decode(&private_b64).expect("Ivalid private key base64");
    let public_pem = general_purpose::STANDARD.decode(&public_b64).expect("Ivalid public key base64");

    Keys::new(private_pem, public_pem)
});
