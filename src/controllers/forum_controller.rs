use actix_web::{web::{self}, Responder};

use crate::{
  structs::main_struct::AppState,
  helpers::response::response_json,
  repositories::forum_repository::*
};

#[doc = "Get all forum"]
pub async fn get_forum(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let data = fetch_forum_data(state.db.clone(), id).await;

  response_json(
    "success".to_string(),
    "Successfully retrieved forum".to_string(),
    data
  )
}