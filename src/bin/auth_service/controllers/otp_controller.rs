use otp::make_hmac_otp;
use validator::Validate;
use serde_json::{Value, json};
use actix_web::{web, Responder};

use crate::{
  schemas::otp_schema::*,
  helpers::{
    token::*,
    mailer::send_email,
    parse::string_to_vec
  },
  repositories::{
    otp_repository::*,
    auth_repository::{
      update_verified_status,
      fetch_user_data_by_email
    }
  }
};

use scholarmate_api::{
  structs::main_struct::AppState,
  repositories::main_repository::check_data,
  helpers::{
    parse::get_email_parts,
    response::response_json
  },
};

pub async fn add_otp(state: web::Data<AppState>, body: web::Json<OTPSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return response_json(
      String::from("failed"),
      String::from("please fill all fields"),
      vec![data]
    )
  }

  let otp: u32 = make_hmac_otp(get_email_parts(body.email.as_str())[0], 1).unwrap();
  let pure_token = generate_token("bert-base-cased", body.email.to_owned()).await.unwrap_or(Vec::new());
  let token = pure_token.iter().map(|x| x.to_string() + ",").collect::<String>();

  let name = get_email_parts(body.email.as_str())[0];
  let reciever = format!("{} <{}>", name, body.email.to_owned());

  let query_str = format!("select 1 from tokens where token = '{}' or email = '{}'", token.to_owned(), body.email.to_owned());
  let token_exist = check_data(state.db.clone(), query_str.as_str()).await;

  if token_exist {
    delete_otp_data(state.db.clone(), token.to_owned()).await;
  }

  let _ = insert_otp_data(state.db.clone(), token.to_owned(), otp.to_string(), body.email.to_owned()).await;  

  match send_email(reciever, String::from("Your OTP"), otp.to_string()).await {
    Ok(_) => response_json(
      String::from("success"),
      String::from("successfully send email"),
      vec![json!({
        "token": token,
        "otp": otp
      })]
    ),
    Err(_) => response_json(
      String::from("failed"),
      String::from("failed to send email"),
      vec![]
    )
  }
}

pub async fn update_otp(state: web::Data<AppState>, body: web::Json<DetailOTPSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return response_json(
      String::from("failed"),
      String::from("please fill all fields"),
      vec![data]
    )
  }

  let query_str = format!("select 1 from tokens where token = '{}' or email = '{}'", body.token.to_owned(), body.email.to_owned());
  let token_exist = check_data(state.db.clone(), query_str.as_str()).await;

  if token_exist {
    delete_otp_data(state.db.clone(), body.token.to_owned()).await;
  }

  let otp: u32 = make_hmac_otp(get_email_parts(body.email.as_str())[0], 1).unwrap();
  let pure_token = generate_token("bert-base-cased", body.email.to_owned()).await.unwrap_or(Vec::new());
  let token = pure_token.iter().map(|x| x.to_string() + ",").collect::<String>();

  let name = get_email_parts(body.email.as_str())[0];
  let reciever = format!("{} <{}>", name, body.email.to_owned());

  let new_query_str = format!("select 1 from tokens where token = '{}' or email = '{}'", token.to_owned(), body.email.to_owned());
  let token_exist = check_data(state.db.clone(), new_query_str.as_str()).await;

  if token_exist {
    delete_otp_data(state.db.clone(), token.to_owned()).await;
  }

  let _ = insert_otp_data(state.db.clone(), token.to_owned(), otp.to_string(), body.email.to_owned()).await;  

  match send_email(reciever, String::from("Your OTP"), otp.to_string()).await {
    Ok(_) => response_json(
      String::from("success"),
      String::from("successfully send email"),
      vec![json!({
        "token": token,
        "otp": otp
      })]
    ),
    Err(_) => response_json(
      String::from("failed"),
      String::from("failed to send email"),
      vec![]
    )
  }
}

pub async fn delete_otp(state: web::Data<AppState>, body: web::Json<DetailOTPSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return response_json(
      String::from("failed"),
      String::from("please fill all fields"),
      vec![data]
    )
  }

  let query_str = format!("select 1 from tokens where token = '{}' or email = '{}'", body.token.to_owned(), body.email.to_owned());
  let token_exist = check_data(state.db.clone(), query_str.as_str()).await;

  if token_exist {
    delete_otp_data(state.db.clone(), body.token.to_owned()).await;

    response_json(
      String::from("success"),
      String::from("successfully delete otp"),
      vec![]
    )
  } else {
    response_json(
      String::from("failed"),
      String::from("otp not found"),
      vec![]
    )
  }
}

pub async fn validate_otp(state: web::Data<AppState>, body: web::Json<OTPVerificationSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return response_json(
      String::from("failed"),
      String::from("please fill all fields"),
      vec![data]
    )
  }

  let query_str = format!("select 1 from tokens where token = '{}' or email = '{}'", body.token.to_owned(), body.email.to_owned());
  let token_exist = check_data(state.db.clone(), query_str.as_str()).await;

  if token_exist {
    match fetch_otp_data_by_token(state.db.clone(), body.token.to_owned()).await {
      Some(data) => {
        let current_token = decode_token("bert-base-cased", string_to_vec(data.token.as_str())).await.unwrap();
        let body_token = decode_token("bert-base-cased", string_to_vec(body.token.to_owned().as_str())).await.unwrap();

        if current_token == body_token {
          let current_otp = data.otp;
          let body_otp = body.otp.to_owned();

          if current_otp == body_otp {
            delete_otp_data(state.db.clone(), body.token.to_owned()).await;

            match fetch_user_data_by_email(state.db.clone(), body.email.to_owned()).await {
              Some(_) => {
                let _ = update_verified_status(state.db.clone(), body.email.to_owned()).await;

                response_json(
                  String::from("success"),
                  String::from("successfully verified otp"),
                  vec![]
                )
              }
              None => response_json(
                String::from("failed"),
                String::from("user not found"),
                vec![]
              )
            }
          } else {
            response_json(
              String::from("failed"),
              String::from("otp is invalid"),
              vec![]
            )
          }
        } else {
          response_json(
            String::from("failed"),
            String::from("token is invalid"),
            vec![]
          )
        }
      }
      None => response_json(
        String::from("failed"),
        String::from("token not found"),
        vec![]
      )
    }
  } else {
    response_json(
      String::from("failed"),
      String::from("otp not found"),
      vec![]
    )
  }
}