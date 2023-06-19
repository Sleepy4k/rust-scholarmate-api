use serde::{Serialize, Deserialize};

#[doc = "Forum model"]
#[derive(Debug, Deserialize, Serialize)]
pub struct ForumModel {
  pub alias: String,
  pub message: String,
  pub image: String,
  pub link: String,
}