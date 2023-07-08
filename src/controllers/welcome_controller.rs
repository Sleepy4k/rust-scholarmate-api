use std::env;
use serde_json::json;
use actix_web::Responder;

use crate::helpers::response::response_json;

#[doc = "Welcome route"]
pub async fn welcome() -> impl Responder {
  let app_name = env::var("APP_NAME").unwrap_or("actix-api".to_string());
  let message = format!("welcome to {}", app_name);
  let data = vec![
    json!({
      "health": "good"
    })
  ];

  response_json(
    "success".to_string(),
    message,
    data
  )
}