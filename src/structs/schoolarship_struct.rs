use serde::{Serialize};

#[doc = "Schoolarship struct"]
#[derive(Debug, Serialize)]
pub struct SchoolarshipStruct {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub major: String,
  pub quantity: i32,
  pub requirement: String,
}