use serde_json::Value;
use actix_web::{web, Responder};

use crate::{
  structs::main_struct::AppState,
  helpers::response::create_response,
  enums::response_enum::ResponseDataEnum,
  repositories::auth_repository::fetch_user_data
};

#[doc = "Get all user"]
pub async fn get_user(state: web::Data<AppState>) -> impl Responder {
  let data = fetch_user_data(state.db.to_owned()).await;
  let body: ResponseDataEnum<Value> = ResponseDataEnum::ArrayValue(data);

  create_response(
    String::from("success"),
    String::from("successfully retrieved user"),
    body.get_value()
  )
}