use std::fs;

use lettre::{
    message::{self, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    SmtpTransport, Transport,
};
use lettre_email::error::Error;

use crate::constants::TEACHER_EMAIL;

use super::{
    email_error::{self, EmailError},
    email_generator::{self, EmailType},
};

pub struct EmailHandler {
    pub sender: String,
    pub topic: String,
    pub message: String,
    pub attachment: Option<String>, // file path
    pub password: String,
    pub smtp_server: String,
}

impl EmailHandler {
    pub fn send(email_data: &Self) -> Result<String, EmailError> {
        let mut email_type = EmailType::HomeWorkCodeAsText(email_data.message.clone());
        if let Some(attachment_path) = &email_data.attachment {
            if !fs::metadata(attachment_path).is_ok() {
                return Err(EmailError::AttachmentNotFound);
            }
            email_type = EmailType::HomeWorkCodeAsAttachment;
        }
        if email_data.sender.is_empty() {
            return Err(EmailError::SenderEmailInvalid);
        }
        if email_data.topic.is_empty() {
            return Err(EmailError::NoTopic);
        }

        let email_body = match email_type {
            EmailType::HomeWorkCodeAsText(msg) => {
                email_generator::email_body_generator(EmailType::HomeWorkCodeAsText(msg))
            }
            EmailType::HomeWorkCodeAsAttachment => {
                email_generator::email_body_generator(EmailType::HomeWorkCodeAsAttachment)
            }
        };

        let email = lettre::Message::builder()
            .from(format!("Student <{}>", email_data.sender).parse().unwrap())
            .to(format!("Miroslaw Moscicki <{}>", TEACHER_EMAIL)
                .parse()
                .unwrap())
            .subject(email_data.topic.clone())
            .multipart(
                MultiPart::alternative().singlepart(SinglePart::html(email_body.to_string())),
            )
            .unwrap();
        let creds = Credentials::new(email_data.sender.clone(), email_data.password.clone());
        let mailer = SmtpTransport::relay(&email_data.smtp_server)
            .map_err(|_| EmailError::SenderEmailNotFound)?
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => Ok(String::from("Email sent Successfully")),
            Err(_) => Err(EmailError::UnexpectedError),
        }
    }
}
