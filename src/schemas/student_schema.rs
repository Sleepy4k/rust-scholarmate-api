use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

use crate::helpers::validation::check_if_valid_date;

#[doc = "Student schema"]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct StudentSchema {
  #[validate(length(min = 1, max = 255, message = "first_name is required and must be less than 255 characters"))]
  pub first_name: String,
  #[validate(length(min = 1, max = 255, message = "last_name is required and must be less than 255 characters"))]
  pub last_name: String,
  #[validate(length(min = 1, max = 255, message = "email is required and must be less than 255 characters"), email(message = "email must be a valid email"))]
  pub email: String,
  #[validate(length(min = 1, max = 255, message = "phone is required and must be less than 255 characters"))]
  pub phone: String,
  #[validate(length(min = 1, max = 255, message = "date_of_birth is required and must be less than 255 characters"), custom = "validate_date_of_birth")]
  pub date_of_birth: String,
  #[validate(length(min = 1, max = 255, message = "region is required and must be less than 255 characters"))]
  pub region: String,
  #[validate(length(min = 1, max = 255, message = "register_number is required and must be less than 255 characters"))]
  pub register_number: String,
  #[validate(range(min = 0, max = 700, message = "toefl_score is required and must be less than 700"))]
  pub toefl_score: i32,
  #[validate(range(min = 0, max = 9, message = "ielts_score is required and must be less than 700"))]
  pub ielts_score: i32,
}

#[doc = "Validate date of birth"]
fn validate_date_of_birth(date: &str) -> Result<(), ValidationError> {
  if !check_if_valid_date(date) {
    return Err(ValidationError::new("date_of_birth must be a valid date"));
  }

  Ok(())
}