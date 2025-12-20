use std::env::var;

use crate::services::auth_service::EMAIL_VERIFICATION_EXP_MINUTES;

pub struct VerifyEmail {
    app_name: String,
    username: String,
    verification_url: String,
    expiry_minutes: String,
    support_email: String,
    company_address: String,
    unsubscribe_url: String,
}

impl VerifyEmail {
    pub fn new(username: &str, user_id: &str, token: &str) -> Self {
        Self {
            app_name: var("APP_NAME").expect("APP_NAME missing"),
            username: username.to_owned(),
            verification_url: format!(
                "{}/auth/verify_email?token={}&user_id={}",
                var("DOMAIN").expect("DOMAIN missing"),
                token,
                user_id
            ),
            expiry_minutes: (EMAIL_VERIFICATION_EXP_MINUTES / 60).to_string(),
            support_email: var("SUPPORT_EMAIL").expect("SUPPORT_EMAIL missing"),
            company_address: var("COMPANY_ADDRESS").expect("COMPANY_ADDRESS missing"),
            unsubscribe_url: format!("{}/unsubscribe", var("DOMAIN").expect("DOMAIN missing")),
        }
    }
    pub fn as_array(&self) -> [(&str, &str); 7] {
        [
            ("app_name", &self.app_name),
            ("username", &self.username),
            ("verification_url", &self.verification_url),
            ("expiry_minutes", &self.expiry_minutes),
            ("support_email", &self.support_email),
            ("company_address", &self.company_address),
            ("unsubscribe_url", &self.unsubscribe_url),
        ]
    }
}
