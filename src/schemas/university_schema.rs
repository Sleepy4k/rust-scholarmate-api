use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

#[doc = "University schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UniversitySchema {
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  #[validate(length(min = 1, max = 255))]
  pub major: String,
  #[validate(custom = "validate_positif_number")]
  pub quantity: i32,
  #[validate(length(min = 1, max = 255))]
  pub description: String,
}

#[doc = "Validate positif number"]
fn validate_positif_number(number: i32) -> Result<(), ValidationError> {
  if number < 0 {
    return Err(ValidationError::new("Number must be positif"));
  }

  Ok(())
}