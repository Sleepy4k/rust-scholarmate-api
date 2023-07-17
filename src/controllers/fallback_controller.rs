use serde_json::json;
use actix_web::{HttpRequest, Responder};

use crate::{
  helpers::response::create_response,
  enums::response_enum::ResponseDataEnum
};

#[doc = "Default route for all routes that are not defined"]
pub async fn fallback(request: HttpRequest) -> impl Responder {
  let path = request.path().to_string();
  let method = request.method().to_string();
  let body = ResponseDataEnum::SingleValue(json!({
    "path": path,
    "method": method
  }));

  create_response(
    String::from("not found"),
    String::from("route not found or not implemented"),
    body
  )
}