use reqwest::Client;

use crate::types::{email::Email, error::CustomError, verify_email::VerifyEmail};

use std::env::var;

pub enum EmailTemplateValues {
    VerifyEmailValues(VerifyEmail),
}

pub struct EmailService {}

impl EmailService {
    pub fn prepare_template(
        &self,
        bytes: &[u8],
        ext: &str,
        values: EmailTemplateValues,
    ) -> Result<String, CustomError> {
        if !ext.contains("html") {
            return Err(CustomError::EmailTemplateError);
        }

        // Implement convert to string
        let mut template = match String::from_utf8(bytes.to_owned()) {
            Ok(v) => v,
            Err(_e) => return Err(CustomError::EmailTemplateError),
        };

        // Implement replace all placeholders with values
        match values {
            EmailTemplateValues::VerifyEmailValues(fields) => {
                for (name, value) in fields.as_array() {
                    template = template.replace(&format!("{{{{{}}}}}", name), value)
                }

                Ok(template)
            }
        }
    }

    pub async fn send_transactional_email(&self, email: Email) -> Result<(), CustomError> {
        let client = Client::new();

        let api_key = var("BREVO_API_KEY").expect("BREVO_API_KEY missing");

        let res = client
            .post("https://api.brevo.com/v3/smtp/email")
            .header("api-key", api_key)
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .json(&email)
            .send()
            .await?;

        if res.status().is_success() {
            tracing::info!("Email verification has been sent");
            return Ok(());
        }

        let status = res.status();
        let body = res.text().await.unwrap_or_default();

        tracing::error!("Brevo error {}: {}", status, body);

        Err(CustomError::SendEmailError)
    }
}
