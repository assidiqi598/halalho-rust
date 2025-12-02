use std::env::var;

#[allow(non_snake_case)]
#[derive(Debug, serde::Serialize)]
pub struct Email {
    sender: Person,
    to: Vec<Person>,
    htmlContent: String,
    subject: String,
}

#[derive(Debug, serde::Serialize)]
struct Person {
    name: String,
    email: String,
}

impl Email {

    /// Create email object to be sent
    /// 
    /// recipients are vector of tuples (recipient_name, recipient_email)
    pub fn new(
        recipients: Vec<(&str, &str)>,
        html: String,
        subject: &str,
    ) -> Self {
        let mut recipients_vec: Vec<Person> = Vec::new();

        for rec in recipients.iter() {
            recipients_vec.push(Person {
                name: rec.0.to_owned(),
                email: rec.1.to_owned(),
            });
        }

        Self {
            sender: Person {
                name: var("BREVO_SENDER_NAME").expect("BREVO_SENDER_NAME missing"),
                email: var("BREVO_SENDER_EMAIL").expect("BREVO_SENDER_EMAIL missing"),
            },
            to: recipients_vec,
            htmlContent: html,
            subject: subject.to_owned(),
        }
    }
}
