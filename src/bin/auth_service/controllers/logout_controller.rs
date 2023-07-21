use serde_json::json;
use actix_web::Responder;

use scholarmate_api::{
  enums::response_enum::ResponseDataEnum,
  helpers::response::create_response_with_token
};

#[doc = "Logout user"]
pub async fn logout_index_controller() -> impl Responder {
  create_response_with_token(
    String::from("success"),
    String::from("successfully logged out"),
    ResponseDataEnum::SingleValue(json!({})),
    String::new()
  )
}