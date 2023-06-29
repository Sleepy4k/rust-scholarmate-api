use serde_json::Value;
use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

use crate::{
  models::student_model::*,
  schemas::student_schema::*,
  helpers::parse::{convert_vec_to_values, to_i32}
};

#[doc = "Fetch all student data"]
pub async fn fetch_student_data(pool: Pool<Postgres>) -> Vec<Value> {
  let data = sqlx::query_as!(StudentModel, "select * from students")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Insert new student"]
pub async fn insert_student_data(pool: Pool<Postgres>, body: StudentSchema) -> Vec<Value> {
  let dob = NaiveDate::parse_from_str(body.date_of_birth.to_owned().as_str(), "%Y-%m-%d").unwrap();
  let data = sqlx::query_as!(StudentModel,
    "insert into students (first_name, last_name, email, phone, date_of_birth, region, register_number, toefl_score, ielts_score)
      values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning *",
    body.first_name, body.last_name, body.email, body.phone, dob, body.region, body.register_number, to_i32(Value::String(body.toefl_score)), to_i32(Value::String(body.ielts_score)))
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Fetch student data by id"]
pub async fn fetch_student_data_by_id(pool: Pool<Postgres>, id: i32) -> Option<StudentModel> {
  let data = sqlx::query_as!(StudentModel, "select * from students where id = $1", id)
    .fetch_optional(&pool)
    .await
    .unwrap();

  data
}

#[doc = "Fetch student data by exists column"]
pub async fn fetch_student_data_by_exists_column(pool: Pool<Postgres>, email: String, phone: String, register_number: String) -> Option<StudentModel> {
  let data = sqlx::query_as!(StudentModel, "select * from students where email = $1 or phone = $2 or register_number = $3 limit 1", email, phone, register_number)
    .fetch_optional(&pool)
    .await
    .unwrap();

  data
}

#[doc = "Update student data by id"]
pub async fn update_student_data_by_id(pool: Pool<Postgres>, id: i32, body: StudentSchema) -> Vec<Value> {
  let dob = NaiveDate::parse_from_str(body.date_of_birth.to_owned().as_str(), "%Y-%m-%d").unwrap();
  let data = sqlx::query_as!(StudentModel,
    "update students set first_name = $1, last_name = $2, email = $3, phone = $4, date_of_birth = $5,
      region = $6, register_number = $7, toefl_score = $8, ielts_score = $9 where id = $10 returning *",
    body.first_name, body.last_name, body.email, body.phone, dob, body.region, body.register_number, to_i32(Value::String(body.toefl_score)), to_i32(Value::String(body.ielts_score)), id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Delete student data by id"]
pub async fn delete_student_data_by_id(pool: Pool<Postgres>, id: i32) -> Vec<Value> {
  let data = sqlx::query_as!(StudentModel, "delete from students where id = $1 returning *", id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}
