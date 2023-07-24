use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

use scholarmate_api::helpers::validation::check_if_positif_number;

#[doc = "GeneralChatSchema is the struct that will be used to general chat schema in websocket."]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct GeneralChatSchema {
  #[validate(custom = "validate_positif_number")]
  pub uid: i32
}

#[doc = "DetailChatSchema is the struct that will be used to detail chat schema in websocket."]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct DetailChatSchema {
  #[validate(length(min = 1, max = 255, message = "name is required and must be less than 255 characters"))]
  pub name: String,
  #[validate(length(min = 1, max = 255, message = "sender is required and must be less than 255 characters"))]
  pub sender: String,
  #[validate(custom = "validate_positif_number")]
  pub reciver: i32,
}

#[doc = "Validate positif number"]
fn validate_positif_number(number: i32) -> Result<(), ValidationError> {
  if !check_if_positif_number(number) {
    return Err(ValidationError::new("number is required and must be positif number"));
  }

  Ok(())
}