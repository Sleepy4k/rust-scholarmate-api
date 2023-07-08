use actix_web::Responder;

use scholarmate_api::helpers::response::response_json_with_cookie;

#[doc = "Logout user"]
pub async fn logout() -> impl Responder {
  response_json_with_cookie(
    String::from("success"),
    String::from("successfully logged out"),
    vec![],
    String::new()
  )
}