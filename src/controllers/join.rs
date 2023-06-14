use chrono::NaiveDate;
use serde_json::Value;
use actix_web::{web::{self}, Responder};

use crate::{helpers::{response::*, database::connect_postgres, parse::*, validation::*}, structs::student::*};

#[doc = "Add new student"]
pub async fn post_join(body: web::Json<Value>) -> impl Responder {
  let first_name = to_str(map_get("firstName", body.to_owned()));
  let last_name = to_str(map_get("lastName", body.to_owned()));
  let email = to_str(map_get("email", body.to_owned()));
  let phone = to_str(map_get("phone", body.to_owned()));
  let dob = to_str(map_get("dob", body.to_owned()));
  let region = to_str(map_get("region", body.to_owned()));
  let register_number = to_i32(map_get("nik", body.to_owned()));
  let toefl_score = to_i32(map_get("toefl", body.to_owned()));
  let ielts_score = to_i32(map_get("ielts", body.to_owned()));

  if check_if_empty(first_name.to_owned()) || check_if_empty(last_name.to_owned()) || check_if_empty(email.to_owned()) || check_if_empty(phone.to_owned()) || check_if_empty(dob.to_owned()) || check_if_empty(region.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all the fields".to_string(),
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

  let date_of_birth = NaiveDate::parse_from_str(dob.as_str(), "%Y-%m-%d").unwrap();
  let data = sqlx::query_as!(StudentStruct, "insert into students (first_name, last_name, email, phone, date_of_birth, region, register_number, toefl_score, ielts_score) values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning *", first_name, last_name, email, phone, date_of_birth, region, register_number, toefl_score, ielts_score)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(vec![data]);

  response_json(
    "success".to_string(),
    "Successfully added student".to_string(),
    result
  )
}