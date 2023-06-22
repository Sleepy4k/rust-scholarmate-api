use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

#[doc = "Application schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ApplicationSchema {
  #[validate(length(min = 1, max = 255))]
  pub major: String,
  #[validate(length(min = 1, max = 255))]
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
  if number < 0 {
    return Err(ValidationError::new("Number must be positif"));
  }

  Ok(())
}