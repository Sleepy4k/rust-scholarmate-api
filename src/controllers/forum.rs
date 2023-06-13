use actix_web::Responder;

use crate::{helpers::{response::*, database::connect_postgres, parse::*}, structs::forum::*};

#[doc = "Get all forum"]
pub async fn get_forum() -> impl Responder {
  let pool = connect_postgres().await;
  let data = sqlx::query_as!(ForumStruct, "select * from forums")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved forum".to_string(),
    result
  )
}