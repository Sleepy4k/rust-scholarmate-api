use serde_json::Value;
use actix_web::{web::{self}, Responder};

use crate::{helpers::{response::*, database::connect_postgres, parse::*, validation::*}, structs::university_struct::*};

#[doc = "Get all university"]
pub async fn get_university() -> impl Responder {
  let pool = connect_postgres().await;
  let data = sqlx::query_as!(UniversityStruct, "select * from universities")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved university".to_string(),
    result
  )
}

#[doc = "Add new university"]
pub async fn add_university(body: web::Json<Value>) -> impl Responder {
  let name = to_str(map_get("name", body.to_owned()));
  let description = to_str(map_get("description", body.to_owned()));
  let major = to_str(map_get("major", body.to_owned()));
  let quantity = to_i32(map_get("quantity", body.to_owned()));

  if check_if_empty(name.to_owned()) || check_if_empty(description.to_owned()) || check_if_empty(major.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let pool = connect_postgres().await;

  match sqlx::query!("select * from universities where name = $1 limit 1", name.clone())
    .fetch_optional(&pool)
    .await {
      Ok(Some(_)) => {
        return response_json(
          "failed".to_string(),
          "University already exists".to_string(),
          vec![]
        )
      }
      Ok(None) => (),
      Err(_) => return response_json(
        "error".to_string(),
        "Something went wrong".to_string(),
        vec![]
      )
    };

  let data = sqlx::query_as!(UniversityStruct, "insert into universities (name, description, major, quantity) values ($1, $2, $3, $4) returning *", name, description, major, quantity)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(vec![data]);

  response_json(
    "success".to_string(),
    "Successfully added university".to_string(),
    result
  )
}

#[doc = "Find university by id"]
pub async fn find_university(arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();

  let pool = connect_postgres().await;
  let data = sqlx::query_as!(UniversityStruct, "select * from universities where id = $1", id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  if check_if_empty(result.to_owned()) {
    return response_json(
      "failed".to_string(),
      "University not found".to_string(),
      vec![]
    )
  }

  response_json(
    "success".to_string(),
    "Successfully retrieved university".to_string(),
    result
  )
}

#[doc = "Update university by id"]
pub async fn update_university(body: web::Json<Value>, arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();
  let name = to_str(map_get("name", body.to_owned()));
  let description = to_str(map_get("description", body.to_owned()));
  let major = to_str(map_get("major", body.to_owned()));
  let quantity = to_i32(map_get("quantity", body.to_owned()));

  if check_if_empty(name.to_owned()) || check_if_empty(description.to_owned()) || check_if_empty(major.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let pool = connect_postgres().await;
  let data = sqlx::query_as!(UniversityStruct, "update universities set name = $1, description = $2, major = $3, quantity = $4 where id = $5 returning *", name, description, major, quantity, id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  if check_if_empty(result.to_owned()) {
    return response_json(
      "failed".to_string(),
      "University not found".to_string(),
      vec![]
    )
  }

  response_json(
    "success".to_string(),
    "Successfully updated university".to_string(),
    result
  )
}

#[doc = "Delete university by id"]
pub async fn delete_university(arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();

  let pool = connect_postgres().await;
  let data = sqlx::query_as!(UniversityStruct, "delete from universities where id = $1 returning *", id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  if check_if_empty(result.to_owned()) {
    return response_json(
      "failed".to_string(),
      "University not found".to_string(),
      vec![]
    )
  }

  response_json(
    "success".to_string(),
    "Successfully deleted university".to_string(),
    result
  )
}