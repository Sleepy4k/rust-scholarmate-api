use serde::{Serialize, Deserialize};

#[doc = "Scholarship model"]
#[derive(Debug, Deserialize, Serialize)]
pub struct ScholarshipModel {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub quantity: i32,
  pub requirement: String,
  pub univ_id: i32,
}

#[doc = "Detail scholarship struct"]
#[derive(Debug, Deserialize, Serialize)]
pub struct DetailScholarshipModel {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub quantity: i32,
  pub requirement: String,
  pub univ_id: i32,
  pub univ_major: String,
  pub univ_name: String,
  pub univ_alias: String,
  pub univ_description: String,
}