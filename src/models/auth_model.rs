use serde::{Serialize, Deserialize};

#[doc = "Filtered user model"]
#[derive(Debug, Deserialize, Serialize)]
pub struct FilteredUserModel {
  pub id: i32,
  pub email: String,
  pub role: String,
}
