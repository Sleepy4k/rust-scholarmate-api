use serde_json::json;
use actix_web::{web, Responder};

use crate::{
  structs::main_struct::AppState,
  helpers::response::create_response,
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  },
  services::auth_service::user_get_service
};

#[doc = "Display a listing of the resource."]
pub async fn user_index_controller(state: web::Data<AppState>) -> impl Responder {
  match user_get_service(state.db.to_owned()).await {
    Ok(data) => create_response(
      String::from("success"),
      String::from("successfully retrieved user data"),
      data
    ),
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