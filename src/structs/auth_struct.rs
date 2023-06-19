use serde::{Serialize, Deserialize};

use crate::structs::student_struct::StudentStruct;

#[doc = "Token struct"]
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenStruct {
  pub id: i32,
  pub role: String,
  pub email: String,
  pub iat: u64,
  pub exp: u64,
}

#[doc = "User struct"]
#[derive(Debug, Deserialize, Serialize)]
pub struct UserStruct {
  pub id: i32,
  pub email: String,
  pub role: String,
}

#[doc = "Detail user struct"]
#[derive(Debug, Serialize)]
pub struct DetailUserStruct {
  pub id: i32,
  pub email: String,
  pub role: String,
  pub student: Option<StudentStruct>,
}

#[doc = "Login struct"]
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginStruct {
  pub email: String,
  pub password: String,
}

#[doc = "Register struct"]
#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterStruct {
  pub email: String,
  pub password: String,
  pub role: String,
}