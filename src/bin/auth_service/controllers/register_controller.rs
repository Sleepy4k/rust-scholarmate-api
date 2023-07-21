use serde_json::{Value, json};
use validator::Validate;
use actix_web::{web, Responder};

use crate::{
  schemas::register_schema::RegisterSchema,
  services::register_service::*
};

use scholarmate_api::{
  structs::main_struct::AppState,
  helpers::response::create_response,
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  },
};

#[doc = "Register new user"]
pub async fn register_index_controller(state: web::Data<AppState>, body: web::Json<RegisterSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match register_index_service(state.db.clone(), body.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully registered"),
        data
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