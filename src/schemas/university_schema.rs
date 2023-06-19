use serde::{Serialize, Deserialize};

#[doc = "University schema"]
#[derive(Debug, Deserialize, Serialize)]
pub struct UniversitySchema {
  pub name: String,
  pub major: String,
  pub quantity: i32,
  pub description: String,
}