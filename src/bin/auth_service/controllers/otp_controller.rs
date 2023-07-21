use validator::Validate;
use serde_json::{Value, json};
use actix_web::{web, Responder};

use crate::{
  schemas::otp_schema::*,
  services::otp_service::*
};

use scholarmate_api::{
  structs::main_struct::AppState,
  helpers::response::create_response,
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

#[doc = "Add new otp"]
pub async fn add_otp(state: web::Data<AppState>, body: web::Json<OTPSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match otp_store_service(state.db.clone(), body.into_inner()).await {
    Ok(data) => create_response(
      String::from("success"),
      String::from("successfully create otp"),
      data
    ),
    Err(err) => {
      match err {
        ErrorEnum::CustomError(_) => create_response(
          String::from("unprocessable entity"),
          err.get_error(),
          ResponseDataEnum::SingleValue(json!({}))
        ),
        _ => create_response(
          String::from("internal server error"),
          err.get_error(),
          ResponseDataEnum::SingleValue(json!({}))
        )
      }
    }
  }
}

#[doc = "Update otp"]
pub async fn update_otp(state: web::Data<AppState>, body: web::Json<DetailOTPSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match otp_update_service(state.db.clone(), body.into_inner()).await {
    Ok(data) => create_response(
      String::from("success"),
      String::from("successfully update otp"),
      data
    ),
    Err(err) => {
      match err {
        ErrorEnum::CustomError(_) => create_response(
          String::from("unprocessable entity"),
          err.get_error(),
          ResponseDataEnum::SingleValue(json!({}))
        ),
        _ => create_response(
          String::from("internal server error"),
          err.get_error(),
          ResponseDataEnum::SingleValue(json!({}))
        )
      }
    }
  }
}

#[doc = "Delete otp"]
pub async fn delete_otp(state: web::Data<AppState>, body: web::Json<DetailOTPSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match otp_destroy_service(state.db.clone(), body.into_inner()).await {
    Ok(data) => create_response(
      String::from("success"),
      String::from("successfully delete otp"),
      data
    ),
    Err(err) => {
      match err {
        ErrorEnum::CustomError(_) => create_response(
          String::from("unprocessable entity"),
          err.get_error(),
          ResponseDataEnum::SingleValue(json!({}))
        ),
        _ => create_response(
          String::from("internal server error"),
          err.get_error(),
          ResponseDataEnum::SingleValue(json!({}))
        )
      }
    }
  }
}

#[doc = "Validate otp"]
pub async fn validate_otp(state: web::Data<AppState>, body: web::Json<OTPVerificationSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match otp_verify_service(state.db.clone(), body.into_inner()).await {
    Ok(data) => create_response(
      String::from("success"),
      String::from("successfully verify otp"),
      data
    ),
    Err(err) => {
      match err {
        ErrorEnum::CustomError(_) => create_response(
          String::from("unprocessable entity"),
          err.get_error(),
          ResponseDataEnum::SingleValue(json!({}))
        ),
        _ => create_response(
          String::from("internal server error"),
          err.get_error(),
          ResponseDataEnum::SingleValue(json!({}))
        )
      }
    }
  }
}