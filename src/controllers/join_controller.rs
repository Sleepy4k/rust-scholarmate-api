use chrono::NaiveDate;
use actix_web::{web::{self}, Responder};

use crate::{helpers::{response::response_json, parse::convert_vec_to_values, validation::check_if_empty}, structs::{student_struct::*, main_struct::*}};

#[doc = "Add new student"]
pub async fn post_join(state: web::Data<AppState>, body: web::Json<StudentBodyStruct>) -> impl Responder {
  let first_name = body.first_name.to_owned();
  let last_name = body.last_name.to_owned();
  let email = body.email.to_owned();
  let phone = body.phone.to_owned();
  let date_of_birth = body.date_of_birth.to_owned().to_string();
  let region = body.region.to_owned();
  let register_number = body.register_number.to_owned();
  let toefl_score = body.toefl_score.to_owned();
  let ielts_score = body.ielts_score.to_owned();

  if check_if_empty(first_name.to_owned()) || check_if_empty(last_name.to_owned()) || check_if_empty(email.to_owned()) || check_if_empty(phone.to_owned()) || check_if_empty(date_of_birth.to_owned()) || check_if_empty(region.to_owned()) || check_if_empty(register_number.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all the fields".to_string(),
      vec![]
    )
  }

  let student_exists = sqlx::query_scalar::<_, bool>("select exists(select 1 from students where email = $1 or phone = $2 or register_number = $3) as student_exists")
    .bind(email.clone())
    .bind(phone.clone())
    .bind(register_number.clone())
    .fetch_one(&state.db)
    .await
    .unwrap_or(false);

  if student_exists {
    return response_json(
      "failed".to_string(),
      "Student already exists".to_string(),
      vec![]
    )
  }

  let dob = NaiveDate::parse_from_str(date_of_birth.as_str(), "%Y-%m-%d").unwrap();
  let data = sqlx::query_as!(StudentStruct,
    "insert into students (first_name, last_name, email, phone, date_of_birth, region, register_number, toefl_score, ielts_score)
      values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning *",
    first_name, last_name, email, phone, dob, region, register_number, toefl_score, ielts_score)
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully added student".to_string(),
    result
  )
}

#[doc = "Update student data"]
pub async fn put_join(state: web::Data<AppState>, body: web::Json<StudentBodyStruct>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let first_name = body.first_name.to_owned();
  let last_name = body.last_name.to_owned();
  let email = body.email.to_owned();
  let phone = body.phone.to_owned();
  let date_of_birth = body.date_of_birth.to_owned().to_string();
  let region = body.region.to_owned();
  let register_number = body.register_number.to_owned();
  let toefl_score = body.toefl_score.to_owned();
  let ielts_score = body.ielts_score.to_owned();

  if check_if_empty(first_name.to_owned()) || check_if_empty(last_name.to_owned()) || check_if_empty(email.to_owned()) || check_if_empty(phone.to_owned()) || check_if_empty(date_of_birth.to_owned()) || check_if_empty(region.to_owned()) || check_if_empty(register_number.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all the fields".to_string(),
      vec![]
    )
  }

  let student_exists = sqlx::query_scalar::<_, bool>("select exists(select 1 from students where id = $1) as univ_exists")
    .bind(id.clone())
    .fetch_one(&state.db)
    .await
    .unwrap_or(false);

  if !student_exists {
    return response_json(
      "failed".to_string(),
      "Student not found".to_string(),
      vec![]
    )
  }

  let dob = NaiveDate::parse_from_str(date_of_birth.as_str(), "%Y-%m-%d").unwrap();
  let data = sqlx::query_as!(StudentStruct, 
    "update students set first_name = $1, last_name = $2, email = $3, phone = $4, date_of_birth = $5,
      region = $6, register_number = $7, toefl_score = $8, ielts_score = $9 where id = $10 returning *",
    first_name, last_name, email, phone, dob, region, register_number, toefl_score, ielts_score, id)
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully updated student".to_string(),
    result
  )
}