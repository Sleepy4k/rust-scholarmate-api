use validator::Validate;
use serde_json::{Value, json};
use actix_web::{web, Responder};

use crate::{
  schemas::login_schema::*,
  services::login_service::*
};

use scholarmate_api::{
  structs::main_struct::AppState,
  helpers::response::{
    create_response,
    create_response_with_token
  },
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

#[doc = "Verify user credentials and return token"]
pub async fn login_index_controller(state: web::Data<AppState>, body: web::Json<LoginSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match login_index_service(state.db.clone(), body.into_inner()).await {
    Ok(data) => {
      create_response_with_token(
        String::from("success"),
        String::from("successfully logged in"),
        data.0,
        data.1
      )
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(message) => {
          create_response(
            String::from("unprocessable entity"),
            message,
            ResponseDataEnum::SingleValue(json!({}))
          )
        },
        _ => {
          create_response(
            String::from("internal server error"),
            err.get_error(),
            ResponseDataEnum::SingleValue(json!({}))
          )
        }
      }
    }
  }
}