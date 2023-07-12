use validator::Validate;
use serde::{Serialize, Deserialize};

#[doc = "Register schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterSchema {
  #[validate(length(min = 1, max = 255, message = "email is required and must be less than 255 characters"), email(message = "email must be a valid email"))]
  pub email: String,
  #[validate(length(min = 1, max = 255, message = "password is required and must be less than 255 characters"))]
  pub password: String,
  #[validate(length(min = 1, max = 255, message = "confirm_password is required and must be less than 255 characters"), must_match(other = "password", message = "confirm_password must match with password"))]
  pub password_confirmation: String,
}