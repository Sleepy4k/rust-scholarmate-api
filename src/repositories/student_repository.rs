use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

use crate::{
  models::student_model::*,
  enums::error_enum::ErrorEnum,
  schemas::student_schema::StudentSchema
};

#[doc = "Get all student data."]
pub async fn get_student_data(pool: Pool<Postgres>) -> anyhow::Result<Vec<StudentModel>, ErrorEnum> {
  match sqlx::query_as!(StudentModel, "select * from students")
    .fetch_all(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Create an student data."]
pub async fn create_student_data(pool: Pool<Postgres>, body: StudentSchema) -> anyhow::Result<StudentModel, ErrorEnum> {
  let dob = NaiveDate::parse_from_str(body.date_of_birth.to_owned().as_str(), "%Y-%m-%d").unwrap();
  
  match sqlx::query_as!(StudentModel,
    "insert into students (first_name, last_name, email, phone, date_of_birth, region, register_number, toefl_score, ielts_score)
      values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning *",
      body.first_name, body.last_name, body.email, body.phone, dob,
      body.region, body.register_number, body.toefl_score, body.ielts_score)
    .fetch_one(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Find an student data by id."]
pub async fn find_student_data(pool: Pool<Postgres>, id: i32) -> anyhow::Result<StudentModel, ErrorEnum> {
  match sqlx::query_as!(StudentModel, "select * from students where id = $1 limit 1", id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("student not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Find an student data by exists column."]
pub async fn find_student_data_by_exists_column(pool: Pool<Postgres>, email: String, phone: String, register_number: String) -> anyhow::Result<StudentModel, ErrorEnum> {
  match sqlx::query_as!(StudentModel, "select * from students where email = $1 or phone = $2 or register_number = $3 limit 1", email, phone, register_number)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("student not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Update an existing student data."]
pub async fn update_student_data(pool: Pool<Postgres>, id: i32, body: StudentSchema) -> anyhow::Result<StudentModel, ErrorEnum> {
  let dob = NaiveDate::parse_from_str(body.date_of_birth.to_owned().as_str(), "%Y-%m-%d").unwrap();
  
  match sqlx::query_as!(StudentModel,
    "update students set first_name = $1, last_name = $2, email = $3, phone = $4, date_of_birth = $5,
      region = $6, register_number = $7, toefl_score = $8, ielts_score = $9 where id = $10 returning *",
    body.first_name, body.last_name, body.email, body.phone, dob, body.region, body.register_number, body.toefl_score, body.ielts_score, id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("student not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Delete student data by id"]
pub async fn delete_student_data(pool: Pool<Postgres>, id: i32) -> anyhow::Result<StudentModel, ErrorEnum> {
  match sqlx::query_as!(StudentModel,
    "delete from students where id = $1 returning *", id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("student not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}
