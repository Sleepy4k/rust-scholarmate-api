use sqlx::FromRow;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[doc = "Student struct"]
#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct StudentStruct {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub phone: String,
  pub date_of_birth: NaiveDate,
  pub region: String,
  pub register_number: String,
  pub toefl_score: i32,
  pub ielts_score: i32,
}

#[doc = "Student body struct"]
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentBodyStruct {
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub phone: String,
  pub date_of_birth: NaiveDate,
  pub region: String,
  pub register_number: String,
  pub toefl_score: i32,
  pub ielts_score: i32,
}