use actix_web::Responder;

use crate::{helpers::{response::*, database::connect_postgres, parse::*}, structs::auth_struct::*};

#[doc = "Get all user"]
pub async fn get_user() -> impl Responder {
  let pool = connect_postgres().await;
  let data = sqlx::query_as!(UserStruct, "select id, email, role from users")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved user".to_string(),
    result
  )
}