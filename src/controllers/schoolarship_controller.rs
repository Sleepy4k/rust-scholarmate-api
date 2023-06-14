use serde_json::Value;
use actix_web::{web::{self}, Responder};

use crate::{helpers::{response::*, database::connect_postgres, parse::*, validation::*}, structs::schoolarship_struct::*};

#[doc = "Get all schoolarship"]
pub async fn get_schoolarship() -> impl Responder {
  let pool = connect_postgres().await;
  let data = sqlx::query_as!(SchoolarshipStruct, "select * from schoolarships")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved schoolarship".to_string(),
    result
  )
}

#[doc = "Add new schoolarship"]
pub async fn add_schoolarship(body: web::Json<Value>) -> impl Responder {
  let name = to_str(map_get("name", body.to_owned()));
  let description = to_str(map_get("description", body.to_owned()));
  let major = to_str(map_get("major", body.to_owned()));
  let quantity = to_i32(map_get("quantity", body.to_owned()));
  let requirement = to_str(map_get("requirement", body.to_owned()));

  if check_if_empty(name.to_owned()) || check_if_empty(description.to_owned()) || check_if_empty(major.to_owned()) || check_if_empty(requirement.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let pool = connect_postgres().await;

  match sqlx::query!("select * from schoolarships where name = $1 limit 1", name.clone())
    .fetch_optional(&pool)
    .await {
      Ok(Some(_)) => {
        return response_json(
          "failed".to_string(),
          "Schoolarship already exists".to_string(),
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

  let data = sqlx::query_as!(SchoolarshipStruct, "insert into schoolarships (name, description, major, quantity, requirement) values ($1, $2, $3, $4, $5) returning *", name, description, major, quantity, requirement)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully added schoolarship".to_string(),
    result
  )
}

#[doc = "Find schoolarship by id"]
pub async fn find_schoolarship(arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();

  let pool = connect_postgres().await;
  let data = sqlx::query_as!(SchoolarshipStruct, "select * from schoolarships where id = $1", id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  if check_if_empty(result.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Schoolarship not found".to_string(),
      vec![]
    )
  }

  response_json(
    "success".to_string(),
    "Successfully retrieved schoolarship".to_string(),
    result
  )
}

#[doc = "Update schoolarship by id"]
pub async fn update_schoolarship(body: web::Json<Value>, arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();
  let name = to_str(map_get("name", body.to_owned()));
  let description = to_str(map_get("description", body.to_owned()));
  let major = to_str(map_get("major", body.to_owned()));
  let quantity = to_i32(map_get("quantity", body.to_owned()));
  let requirement = to_str(map_get("requirement", body.to_owned()));

  if check_if_empty(name.to_owned()) || check_if_empty(description.to_owned()) || check_if_empty(major.to_owned()) || check_if_empty(requirement.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let pool = connect_postgres().await;
  let data = sqlx::query_as!(SchoolarshipStruct, "update schoolarships set name = $1, description = $2, major = $3, quantity = $4, requirement = $5 where id = $6 returning *", name, description, major, quantity, requirement, id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  if check_if_empty(result.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Schoolarship not found".to_string(),
      vec![]
    )
  }

  response_json(
    "success".to_string(),
    "Successfully updated schoolarship".to_string(),
    result
  )
}

#[doc = "Delete schoolarship by id"]
pub async fn delete_schoolarship(arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();

  let pool = connect_postgres().await;
  let data = sqlx::query_as!(SchoolarshipStruct, "delete from schoolarships where id = $1 returning *", id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  if check_if_empty(result.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Schoolarship not found".to_string(),
      vec![]
    )
  }

  response_json(
    "success".to_string(),
    "Successfully deleted schoolarship".to_string(),
    result
  )
}