use std::env;
use actix_web::Responder;
use serde_json::{json, Value};

use crate::{
  helpers::response::create_response,
  enums::response_enum::ResponseDataEnum
};

#[doc = "Welcome route"]
pub async fn welcome() -> impl Responder {
  let app_name = env::var("APP_NAME").unwrap_or("actix-api".to_string());
  let message = format!("welcome to {}", app_name);
  let body: ResponseDataEnum<Value> = ResponseDataEnum::SingleValue(json!({
    "health": "good"
  }));

  create_response(
    String::from("success"),
    message,
    body.get_value()
  )
}