mod constants;
mod email;

use core::panic;
use std::io::{self, Write};

use email::email_handler::EmailHandler;
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_email_handler_from_user() -> EmailHandler {
    let sender = read_input("Enter sender email: ");
    let topic = read_input("Enter email topic: ");
    let message = read_input("Enter email message: ");
    let attachment = {
        let attachment_path = read_input("Enter attachment path (leave empty if none): ");
        if attachment_path.is_empty() {
            None
        } else {
            Some(attachment_path)
        }
    };
    let password = read_input("Enter email password: ");
    let smtp_server = read_input("Enter smtp server: ");

    EmailHandler {
        sender,
        topic,
        message,
        attachment,
        password,
        smtp_server,
    }
}
fn main() {
    let email_handler = get_email_handler_from_user();

    println!("Collected Email Data:");
    println!("Sender: {}", email_handler.sender);
    println!("Topic: {}", email_handler.topic);
    println!("Message: {}", email_handler.message);
    println!(
        "Attachment: {}",
        email_handler
            .attachment
            .as_deref()
            .unwrap_or("No attachment provided")
    );
    println!("Password: {}", email_handler.password);

    if read_input("Do you wish to continue [y/n]") == "n" {
        panic!("Email not send, user exited");
    }

    match EmailHandler::send(&email_handler) {
        Ok(success_message) => println!("{}", success_message),
        Err(error) => match error {
            email::email_error::EmailError::SenderEmailInvalid => {
                println!("Error: The sender email is invalid.")
            }
            email::email_error::EmailError::SenderEmailNotFound => {
                println!("Error: The sender email was not found.")
            }
            email::email_error::EmailError::ReceiverEmailInvalid => {
                println!("Error: The receiver email is invalid.")
            }
            email::email_error::EmailError::ReceiverEmailNotFound => {
                println!("Error: The receiver email was not found.")
            }
            email::email_error::EmailError::NoTopic => {
                println!("Error: The email topic is missing.")
            }
            email::email_error::EmailError::AttachmentInvalid => {
                println!("Error: The attachment is invalid.")
            }
            email::email_error::EmailError::AttachmentNotFound => {
                println!("Error: The attachment file was not found.")
            }
            email::email_error::EmailError::PasswordInvalid => {
                println!("Error: The email password is invalid.")
            }
            email::email_error::EmailError::UnexpectedError => {
                println!("Error: Unexpected error")
            }
        },
    }
}
