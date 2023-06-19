use serde::{Serialize, Deserialize};

#[doc = "Application schema"]
#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationSchema {
  pub major: String,
  pub status: String,
  pub univ_id: i32,
  pub student_id: i32,
  pub scholarship_id: i32,
}