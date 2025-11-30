use crate::error::CustomError;

pub struct VerifyEmail {
    app_name: String,
    user_first_name: String,
    verification_url: String,
    expiry_minutes: String,
    support_email: String,
    company_address: String,
    unsubscribe_url: String
}

pub enum EmailTemplateValues {
    VerifyEmailValues(VerifyEmail)
}

pub fn prepare_template(bytes: bytes::Bytes, ext: &str, values: EmailTemplateValues) -> Result<String, CustomError> {
    if !ext.contains("html") {
        return Err(CustomError::EmailTemplateError);
    }

    // Implement convert to string
    // Implement replace all placeholders with values
}