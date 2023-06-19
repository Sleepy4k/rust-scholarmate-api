use serde::{Serialize, Deserialize};

#[doc = "Login schema"]
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginSchema {
  pub email: String,
  pub password: String,
}

#[doc = "Register schema"]
#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterSchema {
  pub email: String,
  pub password: String,
  pub role: String,
}