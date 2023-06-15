use actix_web::{web::{self}, Responder};

use crate::{helpers::{response::*, database::connect_postgres, parse::*}, structs::forum_struct::*};

#[doc = "Get all forum"]
pub async fn get_forum(arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();
  let pool = connect_postgres().await;
  let data = sqlx::query_as!(ForumStruct,
    "select universities.alias, forums.message, universities.image, universities.link from forums
      join universities on forums.univ_id = universities.id where student_id = $1
      order by forums.id desc limit 5"
    , id)
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