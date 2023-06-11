use serde::{Serialize};

#[doc = "Detail user struct"]
#[derive(Debug, Serialize)]
pub struct UniversityStruct {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub major: String,
  pub quantity: i32,
}