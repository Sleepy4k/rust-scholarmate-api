use std::env;
use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

pub async fn send_email(reciever: String, subject: String, body: String) -> anyhow::Result<(), String> {
  let email = Message::builder()
    .from("Scholarmate Admin <pandu300478@gmail.com>".parse().unwrap())
    .to(reciever.parse().unwrap())
    .subject(subject)
    .header(ContentType::TEXT_PLAIN)
    .body(body)
    .unwrap();

  let smtp_username = env::var("SMTP_USERNAME").unwrap_or(String::from("username"));
  let smtp_password = env::var("SMTP_PASSWORD").unwrap_or(String::from("password"));

  let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

  let smtp_host = env::var("SMTP_HOST").unwrap_or(String::from("smtp.gmail.com"));
  let mailer = SmtpTransport::relay(smtp_host.as_str())
    .unwrap()
    .credentials(creds)
    .build();

  match mailer.send(&email) {
    Ok(_) => Ok(()),
    Err(_) => Err(String::from("failed to send email"))
  }
}