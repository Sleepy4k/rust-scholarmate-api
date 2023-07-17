use std::env;
use serde_json::json;
use actix_web::Responder;

use crate::{
  helpers::response::create_response,
  enums::response_enum::ResponseDataEnum
};

#[doc = "Welcome route"]
pub async fn welcome() -> impl Responder {
  let app_name = env::var("APP_NAME").unwrap_or("actix-api".to_string());
  let message = format!("welcome to {}", app_name);
  let body = ResponseDataEnum::SingleValue(json!({
    "health": "good"
  }));

  create_response(
    String::from("success"),
    message,
    body
  )
}