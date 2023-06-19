use std::env;
use actix_web::Responder;

use crate::helpers::response::response_json;

#[doc = "Welcome route"]
pub async fn welcome() -> impl Responder {
  let app_name = env::var("APP_NAME").unwrap_or("actix-api".to_string());
  let message = format!("welcome to {} API", app_name);

  response_json(
    "success".to_string(),
    message,
    vec![]
  )
}