use serde::{Serialize, Deserialize};

use crate::models::student_model::StudentModel;

#[doc = "User model"]
#[derive(Debug, Deserialize, Serialize)]
pub struct UserModel {
  pub id: i32,
  pub email: String,
  pub role: String,
  pub password: String,
}

#[doc = "Filtered user model"]
#[derive(Debug, Deserialize, Serialize)]
pub struct FilteredUserModel {
  pub id: i32,
  pub email: String,
  pub role: String,
}

#[doc = "Detail user model"]
#[derive(Debug, Serialize)]
pub struct DetailUserModel {
  pub id: i32,
  pub email: String,
  pub role: String,
  pub student: Option<StudentModel>,
}
