use serde::{Serialize, Deserialize};

#[doc = "Application model"]
#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationModel {
  pub id: i32,
  pub student_id: i32,
  pub univ_id: i32,
  pub status: String,
  pub major: String,
}

#[doc = "Detail application model"]
#[derive(Debug, Deserialize, Serialize)]
pub struct DetailApplicationModel {
  pub id: i32,
  pub univ_id: i32,
  pub name: String,
  pub major: String,
  pub image: String,
  pub status: String,
}