use serde::{Serialize, Deserialize};

#[doc = "Scholarship schema"]
#[derive(Debug, Deserialize, Serialize)]
pub struct ScholarshipSchema {
  pub name: String,
  pub major: String,
  pub univ_id: i32,
  pub quantity: i32,
  pub description: String,
  pub requirement: String,
}