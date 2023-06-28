use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

use crate::helpers::validation::check_if_valid_role;

#[doc = "Login schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginSchema {
  #[validate(length(min = 1, max = 255, message = "email is required and must be less than 255 characters"), email(message = "email must be a valid email"))]
  pub email: String,
  #[validate(length(min = 1, max = 255, message = "password is required and must be less than 255 characters"))]
  pub password: String,
}

#[doc = "Register schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterSchema {
  #[validate(length(min = 1, max = 255, message = "email is required and must be less than 255 characters"), email(message = "email must be a valid email"))]
  pub email: String,
  #[validate(length(min = 1, max = 255, message = "role is required and must be less than 255 characters"), custom = "validate_role_enum")]
  pub role: String,
  #[validate(length(min = 1, max = 255, message = "password is required and must be less than 255 characters"))]
  pub password: String,
  #[validate(length(min = 1, max = 255, message = "confirm_password is required and must be less than 255 characters"), must_match(other = "password", message = "confirm_password must match with password"))]
  pub password_confirmation: String,
}

#[doc = "Validate role enum"]
fn validate_role_enum(role: &str) -> Result<(), ValidationError> {
  if !check_if_valid_role(role) {
    return Err(ValidationError::new("role must be a valid role"));
  }

  Ok(())
}