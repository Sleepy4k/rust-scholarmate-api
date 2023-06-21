use actix_web::{web, Responder};

use crate::{
  structs::main_struct::AppState,
  helpers::response::response_json,
  repositories::auth_repository::fetch_user_data
};

#[doc = "Get all user"]
pub async fn get_user(state: web::Data<AppState>) -> impl Responder {
  let data = fetch_user_data(state.db.to_owned()).await;

  response_json(
    "success".to_string(),
    "Successfully retrieved user".to_string(),
    data
  )
}