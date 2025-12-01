use crate::error::CustomError;

pub struct VerifyEmail {
    pub app_name: String,
    pub username: String,
    pub verification_url: String,
    pub expiry_minutes: String,
    pub support_email: String,
    pub company_address: String,
    pub unsubscribe_url: String,
}

impl VerifyEmail {
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
            Err(e) => return Err(CustomError::EmailTemplateError),
        };

        println!("template:\n{}", template);

        // Implement replace all placeholders with values
        match values {
            EmailTemplateValues::VerifyEmailValues(fields) => {
                for (name, value) in fields.as_array() {
                    template = template.replace(&format!("{{{{{}}}}}", name), value)
                }

                println!("template:\n{}", template);

                Ok(template)
            }
            _ => return Err(CustomError::EmailTemplateError),
        }
    }
}
