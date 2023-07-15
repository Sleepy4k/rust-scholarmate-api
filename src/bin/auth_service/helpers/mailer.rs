use std::env;
use maud::{DOCTYPE, html, PreEscaped};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::{header::ContentType, MultiPart, SinglePart};

#[doc = "Send email to student using smtp server"]
pub async fn send_email(reciever: String, subject: String, header: ContentType, body: String) -> anyhow::Result<(), String> {
  let smtp_name = env::var("SMTP_NAME").unwrap_or(String::from("Scholarmate".to_owned()));
  let smtp_email = env::var("SMTP_EMAIL").unwrap_or(String::from("scholarmate@example.com".to_owned()));
  let smtp_sender = format!("{} <{}>", smtp_name, smtp_email);

  let email = Message::builder()
    .from(smtp_sender.parse().unwrap())
    .to(reciever.parse().unwrap())
    .subject(subject)
    .multipart(
      MultiPart::alternative()
        .singlepart(
          SinglePart::builder()
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Hello, World!"))
        )
        .singlepart(
          SinglePart::builder()
            .header(header)
            .body(body)
        )
    )
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

#[doc = "Create html body for otp"]
pub fn create_html_body(otp: String, reciever: String) -> PreEscaped<String> {
  html! {
    (DOCTYPE)
    html {
      head {
        title { "OTP Page" }
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        style { r#"
          .container {
            overflow: auto;
            line-height: 2;
            font-family: Helvetica,Arial,sans-serif;
          }
          .main {
            width: 80%;
            padding: 20px 0;
            margin: 50px auto;
          }
          .title {
            border-bottom: 5px solid #eee;
          }
          .title-text {
            color: #03baf1;
            font-size: 30px;
            font-weight: 600;
            text-decoration: none;
          }
          .greeting {
            font-size: 15px;
          }
          .otp-code {
            color: #fff;
            margin: 0 auto;
            padding: 0 10px;
            width: max-content;
            border-radius: 4px;
            background: #00466a;
          }
          .regard {
            font-size: 15px;
          }
          .spacer {
            border: none;
            border-top: 5px solid #eee;
          }
          .footer {
            color: #aaa;
            float: right;
            padding: 8px 0:
            line-height: 1;
            font-size: 0.9em;
            font-weight: 300;
          }
        "# }
      }
      body {
        div class="container" {
          div class="main" {
            div class="title" {
              a class="title-text" href="http://localhost:3000" { "Scholarmate" }
            }
            p class="greeting" { "Hello, " (reciever) }
            p { "Thank you for choosing Scholarmate. Use this OTP to complete your Sign Up procedures and verify your account on Scholarmate." }
            h2 class="otp-code" { (otp.to_string()) }
            p class="regard" { "Regards, Team Scholarmate" }
            hr class="spacer" {}
            p class="footer" {
              p { "Scholarmate, Inc." }
              p { "Jawa Tengah, Indonesia" }
            }
          }
        }
      }
    }
  }
}