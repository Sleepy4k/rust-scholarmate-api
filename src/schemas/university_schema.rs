use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

use crate::helpers::validation::check_if_positif_number;

#[doc = "University schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UniversitySchema {
  #[validate(length(min = 1, max = 255, message = "name is required and must be less than 255 characters"))]
  pub name: String,
  #[validate(length(min = 1, max = 255, message = "major is required and must be less than 255 characters"))]
  pub major: String,
  #[validate(custom = "validate_positif_number")]
  pub quantity: i32,
  #[validate(length(min = 1, max = 255, message = "description is required and must be less than 255 characters"))]
  pub description: String,
  #[validate(length(min = 1, max = 255, message = "image is required and must be less than 255 characters"))]
  pub image: String,
  #[validate(length(min = 1, max = 255, message = "link is required and must be less than 255 characters"))]
  pub link: String,
  #[validate(length(min = 1, max = 255, message = "alias is required and must be less than 255 characters"))]
  pub alias: String,
}

#[doc = "Validate positif number"]
fn validate_positif_number(number: i32) -> Result<(), ValidationError> {
  if !check_if_positif_number(number) {
    return Err(ValidationError::new("number is required and must be positif number"));
  }

  Ok(())
}