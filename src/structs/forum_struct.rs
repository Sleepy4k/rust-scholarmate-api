use serde::{Serialize, Deserialize};

#[doc = "Forum struct"]
#[derive(Debug, Deserialize, Serialize)]
pub struct ForumStruct {
  pub alias: String,
  pub message: String,
  pub image: String,
  pub link: String,
}