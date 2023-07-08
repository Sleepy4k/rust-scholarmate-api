use validator::Validate;
use serde::{Serialize, Deserialize};

#[doc = "Login schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginSchema {
  #[validate(length(min = 1, max = 255, message = "email is required and must be less than 255 characters"), email(message = "email must be a valid email"))]
  pub email: String,
  #[validate(length(min = 1, max = 255, message = "password is required and must be less than 255 characters"))]
  pub password: String,
}