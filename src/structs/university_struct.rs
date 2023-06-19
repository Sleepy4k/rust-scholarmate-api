use serde::{Serialize, Deserialize};

#[doc = "University struct"]
#[derive(Debug, Deserialize, Serialize)]
pub struct UniversityStruct {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub major: String,
  pub quantity: i32,
  pub image: String,
  pub link: String,
  pub alias: String,
}

#[doc = "University body struct"]
#[derive(Debug, Deserialize, Serialize)]
pub struct UniversityBodyStruct {
  pub name: String,
  pub major: String,
  pub quantity: i32,
  pub description: String,
}