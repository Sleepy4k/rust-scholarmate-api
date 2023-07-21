use serde_json::json;
use otp::make_hmac_otp;
use sqlx::{Pool, Postgres};
use lettre::message::header::ContentType;

use crate::{
  schemas::otp_schema::*,
  repositories::{
    otp_repository::*,
    auth_repository::update_user_data
  },
  helpers::{
    token::*,
    mailer::*
  }
};

use scholarmate_api::{
  helpers::parse::get_email_parts,
  repositories::main_repository::check_data,
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

#[doc = "Store a newly created resource in storage."]
pub async fn otp_store_service(db: Pool<Postgres>, body: OTPSchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_user_exist = format!("select 1 from users where email = '{}' and role = 'user' and verified = 'false'", body.email);
  let is_user_exist = check_data(db.clone(), query_is_user_exist.as_str()).await;

  if is_user_exist.is_err() {
    return Err(is_user_exist.err().unwrap());
  } else if !is_user_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("user not exist or not a user")));
  }

  let user_name = get_email_parts(body.email.as_str())[0];
  let user_otp = make_hmac_otp(user_name, 1).unwrap();
  let user_token = generate_token(body.email.to_owned()).await;
  let converted_user_token = user_token.iter().map(|x| x.to_string() + ",").collect::<String>();

  let reciever = format!("{} <{}>", user_name, body.email);
  let query_is_token_exist = format!("select 1 from tokens where token = '{}' and email = '{}'", converted_user_token, body.email.to_owned());
  let is_token_exist = check_data(db.clone(), query_is_token_exist.as_str()).await;

  if is_token_exist.is_err() {
    return Err(is_token_exist.err().unwrap());
  } else if is_token_exist.unwrap() {
    match delete_token_data(db.clone(), converted_user_token.to_owned(), body.email.to_owned()).await {
      Ok(_) => (),
      Err(err) => {
        return Err(err);
      }
    }
  }
  
  let create_body = OTPVerificationSchema {
    token: converted_user_token.to_owned(),
    otp: user_otp.to_string(),
    email: body.email.to_owned()
  };

  match create_token_data(db.clone(), create_body).await {
    Ok(token_data) => {
      let email_body = create_html_body(user_otp.to_string(), user_name.to_string());

      match send_email(reciever, format!("{} is your verification code", user_otp.to_string()), ContentType::TEXT_HTML, email_body.into_string()).await {
        Ok(_) => {
        let body = ResponseDataEnum::SingleValue(json!({
          "token": token_data.token,
          "email": token_data.email
        }));

          Ok(body)
        },
        Err(err) => {
          Err(ErrorEnum::CustomError(err.to_string()))
        }
      }
    },
    Err(err) => Err(err)
  }
}

#[doc = "Update the specified resource in storage."]
pub async fn otp_update_service(db: Pool<Postgres>, body: DetailOTPSchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_user_exist = format!("select 1 from users where email = '{}' and role = 'user' and verified = 'false'", body.email.to_owned());
  let is_user_exist = check_data(db.clone(), query_is_user_exist.as_str()).await;

  if is_user_exist.is_err() {
    return Err(is_user_exist.err().unwrap());
  } else if !is_user_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("user not found or not a user")));
  }

  let user_name = get_email_parts(body.email.as_str())[0];
  let user_otp = make_hmac_otp(user_name, 1).unwrap();
  let user_token = generate_token(body.email.to_owned()).await;
  let converted_user_token = user_token.iter().map(|x| x.to_string() + ",").collect::<String>();

  let reciever = format!("{} <{}>", user_name, body.email);
  let query_is_token_exist = format!("select 1 from tokens where token = '{}' or email = '{}'", converted_user_token, body.email.to_owned());
  let is_token_exist = check_data(db.clone(), query_is_token_exist.as_str()).await;

  if is_token_exist.is_err() {
    return Err(is_token_exist.err().unwrap());
  } else if is_token_exist.unwrap() {
    match delete_token_data(db.clone(), converted_user_token.to_owned(), body.email.to_owned()).await {
      Ok(_) => (),
      Err(err) => {
        return Err(err);
      }
    }
  }
  
  let create_body = OTPVerificationSchema {
    token: converted_user_token.to_owned(),
    otp: user_otp.to_string(),
    email: body.email.to_owned()
  };

  match create_token_data(db.clone(), create_body).await {
    Ok(token_data) => {
      let email_body = create_html_body(user_otp.to_string(), user_name.to_string());

      match send_email(reciever, format!("{} is your verification code", user_otp.to_string()), ContentType::TEXT_HTML, email_body.into_string()).await {
        Ok(_) => {
        let body = ResponseDataEnum::SingleValue(json!({
          "token": token_data.token,
          "email": token_data.email
        }));

          Ok(body)
        },
        Err(err) => {
          Err(ErrorEnum::CustomError(err.to_string()))
        }
      }
    },
    Err(err) => Err(err)
  }
}

#[doc = "Remove the specified resource from storage."]
pub async fn otp_destroy_service(db: Pool<Postgres>, body: DetailOTPSchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_token_exist = format!("select 1 from tokens where token = '{}' or email = '{}'", body.token.to_owned(), body.email.to_owned());
  let is_token_exist = check_data(db.clone(), query_is_token_exist.as_str()).await;

  if is_token_exist.is_err() {
    return Err(is_token_exist.err().unwrap());
  } else if !is_token_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("token not exist")));
  }

  match delete_token_data(db.clone(), body.token.to_owned(), body.email.to_owned()).await {
    Ok(_) => {
      let body = ResponseDataEnum::SingleValue(json!({}));

      Ok(body)
    },
    Err(err) => Err(err)
  }
}

#[doc = "Verify the specified resource from storage."]
pub async fn otp_verify_service(db: Pool<Postgres>, body: OTPVerificationSchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_token_exist = format!("select 1 from tokens where token = '{}' or email = '{}'", body.token.to_owned(), body.email.to_owned());
  let is_token_exist = check_data(db.clone(), query_is_token_exist.as_str()).await;

  if is_token_exist.is_err() {
    return Err(is_token_exist.err().unwrap());
  } else if !is_token_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("token not exist")));
  }

  let query_is_user_exist = format!("select 1 from users where email = '{}' and role = 'user' and verified = 'false'", body.email.to_owned());
  let is_user_exist = check_data(db.clone(), query_is_user_exist.as_str()).await;

  if is_user_exist.is_err() {
    return Err(is_user_exist.err().unwrap());
  } else if !is_user_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("user not found or not a user")));
  }

  let query_is_otp_exist = format!("select 1 from tokens where otp = '{}' and email = '{}'", body.otp.to_owned(), body.email.to_owned());
  let is_otp_exist = check_data(db.clone(), query_is_otp_exist.as_str()).await;

  if is_otp_exist.is_err() {
    return Err(is_otp_exist.err().unwrap());
  } else if !is_otp_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("otp not exist")));
  }

  match delete_token_data(db.clone(), body.token.to_owned(), body.email.to_owned()).await {
    Ok(_) => {
      match update_user_data(db.clone(), body.email.to_owned()).await {
        Ok(_) => {
          let body = ResponseDataEnum::SingleValue(json!({}));
    
          Ok(body)
        },
        Err(err) => {
          Err(err)
        }
      }
    },
    Err(err) => Err(err)
  }
}