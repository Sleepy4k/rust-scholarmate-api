use actix_web::{web::{self}, Responder};

use crate::{helpers::{response::*, database::connect_postgres, parse::*}, structs::application::*};

#[doc = "Get all application"]
pub async fn get_applications() -> impl Responder {
  let pool = connect_postgres().await;
  let data = sqlx::query_as!(ApplicationStruct, "select * from applications")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved application".to_string(),
    result
  )
}

#[doc = "Get user application"]
pub async fn get_my_applications(arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();

  let pool = connect_postgres().await;
  // let data = sqlx::query_as!(ApplicationStruct, "select id, student_id, univ_id from applications a join universities u on a.univ_id = u.id")
  //   .fetch_all(&pool)
  //   .await
  //   .unwrap();

  let data = sqlx::query_as!(ApplicationStruct, "select * from applications where student_id = $1", id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved application".to_string(),
    result
  )
}
