use serde_json::Value;
use chrono::NaiveDate;
use actix_web::{web::{self}, Responder};

use crate::{helpers::{response::*, database::connect_postgres, parse::*, validation::*}, structs::student_struct::*};

#[doc = "Get all student"]
pub async fn get_student() -> impl Responder {
  let pool = connect_postgres().await;
  let data = sqlx::query_as!(StudentStruct, "select * from students")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved student".to_string(),
    result
  )
}

#[doc = "Add new student"]
pub async fn add_student(body: web::Json<Value>) -> impl Responder {
  let first_name = to_str(map_get("first_name", body.to_owned()));
  let last_name = to_str(map_get("last_name", body.to_owned()));
  let email = to_str(map_get("email", body.to_owned()));
  let phone = to_str(map_get("phone", body.to_owned()));
  let date_of_birth = to_str(map_get("date_of_birth", body.to_owned()));
  let region = to_str(map_get("region", body.to_owned()));
  let register_number = to_str(map_get("register_number", body.to_owned()));
  let toefl_score = to_i32(map_get("toefl_score", body.to_owned()));
  let ielts_score = to_i32(map_get("ielts_score", body.to_owned()));

  if check_if_empty(first_name.to_owned()) || check_if_empty(last_name.to_owned()) || check_if_empty(email.to_owned()) || check_if_empty(phone.to_owned()) || check_if_empty(date_of_birth.to_owned()) || check_if_empty(region.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let pool = connect_postgres().await;

  match sqlx::query!("select * from students where email = $1 limit 1", email.clone())
    .fetch_optional(&pool)
    .await {
      Ok(Some(_)) => {
        return response_json(
          "failed".to_string(),
          "Student already exists".to_string(),
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

  let dob = NaiveDate::parse_from_str(date_of_birth.as_str(), "%Y-%m-%d").unwrap();

  let data = sqlx::query_as!(StudentStruct,
    "insert into students (first_name, last_name, email, phone, date_of_birth, region, register_number, toefl_score, ielts_score)
      values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning *",
    first_name, last_name, email, phone, dob, region, register_number, toefl_score, ielts_score)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully added student".to_string(),
    result
  )
}

#[doc = "Find student by id"]
pub async fn find_student(arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();

  let pool = connect_postgres().await;
  let data = sqlx::query_as!(StudentStruct, "select * from students where id = $1", id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  if check_if_empty(result.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Student not found".to_string(),
      vec![]
    )
  }

  response_json(
    "success".to_string(),
    "Successfully retrieved student".to_string(),
    result
  )
}

#[doc = "Update student by id"]
pub async fn update_student(body: web::Json<Value>, arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();
  let first_name = to_str(map_get("first_name", body.to_owned()));
  let last_name = to_str(map_get("last_name", body.to_owned()));
  let email = to_str(map_get("email", body.to_owned()));
  let phone = to_str(map_get("phone", body.to_owned()));
  let date_of_birth = to_str(map_get("date_of_birth", body.to_owned()));
  let region = to_str(map_get("region", body.to_owned()));
  let register_number = to_str(map_get("register_number", body.to_owned()));
  let toefl_score = to_i32(map_get("toefl_score", body.to_owned()));
  let ielts_score = to_i32(map_get("ielts_score", body.to_owned()));

  if check_if_empty(first_name.to_owned()) || check_if_empty(last_name.to_owned()) || check_if_empty(email.to_owned()) || check_if_empty(phone.to_owned()) || check_if_empty(date_of_birth.to_owned()) || check_if_empty(region.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let dob = NaiveDate::parse_from_str(date_of_birth.as_str(), "%Y-%m-%d").unwrap();

  let pool = connect_postgres().await;

  match sqlx::query!("select * from students where id = $1 limit 1", id.clone())
    .fetch_optional(&pool)
    .await {
      Ok(Some(_)) => (),
      Ok(None) => {
        return response_json(
          "failed".to_string(),
          "Student not found".to_string(),
          vec![]
        )
      }
      Err(_) => return response_json(
        "error".to_string(),
        "Something went wrong".to_string(),
        vec![]
      )
    };

  let data = sqlx::query_as!(StudentStruct,
    "update students set first_name = $1, last_name = $2, email = $3, phone = $4, date_of_birth = $5, region = $6, register_number = $7, toefl_score = $8, ielts_score = $9
      where id = $10 returning *",
    first_name, last_name, email, phone, dob, region, register_number, toefl_score, ielts_score, id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  if check_if_empty(result.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Student not found".to_string(),
      vec![]
    )
  }

  response_json(
    "success".to_string(),
    "Successfully updated student".to_string(),
    result
  )
}

#[doc = "Delete student by id"]
pub async fn delete_student(arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();

  let pool = connect_postgres().await;

  match sqlx::query!("select * from students where id = $1 limit 1", id.clone())
    .fetch_optional(&pool)
    .await {
      Ok(Some(_)) => (),
      Ok(None) => {
        return response_json(
          "failed".to_string(),
          "Student not found".to_string(),
          vec![]
        )
      }
      Err(_) => return response_json(
        "error".to_string(),
        "Something went wrong".to_string(),
        vec![]
      )
    };

  let data = sqlx::query_as!(StudentStruct, "delete from students where id = $1 returning *", id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  if check_if_empty(result.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Student not found".to_string(),
      vec![]
    )
  }

  response_json(
    "success".to_string(),
    "Successfully deleted student".to_string(),
    result
  )
}