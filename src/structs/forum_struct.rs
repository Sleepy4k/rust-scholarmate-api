use serde::{Serialize};

#[doc = "Forum struct"]
#[derive(Debug, Serialize)]
pub struct ForumStruct {
  pub id: i32,
  pub name: String,
  pub message: String,
  pub image: Option<String>,
}