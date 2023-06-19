use serde::{Serialize, Deserialize};

#[doc = "University model"]
#[derive(Debug, Deserialize, Serialize)]
pub struct UniversityModel {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub major: String,
  pub quantity: i32,
  pub image: String,
  pub link: String,
  pub alias: String,
}