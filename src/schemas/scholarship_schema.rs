use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

use crate::helpers::validation::check_if_positif_number;

#[doc = "Scholarship schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ScholarshipSchema {
  #[validate(length(min = 1, max = 255, message = "name is required and must be less than 255 characters"))]
  pub name: String,
  #[validate(custom = "validate_positif_number")]
  pub quantity: i32,
  #[validate(length(min = 1, max = 255, message = "description is required and must be less than 255 characters"))]
  pub description: String,
  #[validate(length(min = 1, max = 255, message = "requirement is required and must be less than 255 characters"))]
  pub requirement: String,
  #[validate(custom = "validate_positif_number")]
  pub univ_id: i32,
}

#[doc = "Validate positif number"]
fn validate_positif_number(number: i32) -> Result<(), ValidationError> {
  if !check_if_positif_number(number) {
    return Err(ValidationError::new("number is required and must be positif number"));
  }

  Ok(())
}