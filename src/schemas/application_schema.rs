use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

use crate::helpers::validation::check_if_positif_number;

#[doc = "Application schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ApplicationSchema {
  #[validate(length(min = 1, max = 255, message = "major is required and must be less than 255 characters"))]
  pub major: String,
  #[validate(length(min = 1, max = 255, message = "status is required and must be less than 255 characters"))]
  pub status: String,
  #[validate(custom = "validate_positif_number")]
  pub univ_id: i32,
  #[validate(custom = "validate_positif_number")]
  pub student_id: i32,
  #[validate(custom = "validate_positif_number")]
  pub scholarship_id: i32,
}

#[doc = "Validate positif number"]
fn validate_positif_number(number: i32) -> Result<(), ValidationError> {
  if !check_if_positif_number(number) {
    return Err(ValidationError::new("number is required and must be positif number"));
  }

  Ok(())
}