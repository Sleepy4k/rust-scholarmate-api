use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

#[doc = "Login schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginSchema {
  #[validate(length(min = 1, max = 255), email)]
  pub email: String,
  #[validate(length(min = 1, max = 255))]
  pub password: String,
}

#[doc = "Register schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterSchema {
  #[validate(length(min = 1, max = 255), email)]
  pub email: String,
  #[validate(length(min = 1, max = 255))]
  pub password: String,
  #[validate(length(min = 1, max = 255), custom = "validate_role_enum")]
  pub role: String,
}

#[doc = "Validate role enum"]
fn validate_role_enum(role: &str) -> Result<(), ValidationError> {
  match role {
    "user" => Ok(()),
    "admin" => Ok(()),
    _ => Err(ValidationError::new("Role is not valid"))
  }
}