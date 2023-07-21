use serde_json::Value;
use serde::{Serialize, Deserialize};

#[doc = "User model"]
#[derive(Debug, Deserialize, Serialize)]
pub struct UserModel {
  pub id: i32,
  pub email: String,
  pub role: String,
  pub verified: bool,
  pub password: String,
}

#[doc = "Detail user model"]
#[derive(Debug, Serialize)]
pub struct DetailUserModel {
  pub id: i32,
  pub email: String,
  pub role: String,
  pub verified: bool,
  pub student: Value,
}
