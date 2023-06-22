use regex::Regex;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

#[doc = "Student schema"]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct StudentSchema {
  #[validate(length(min = 1, max = 255))]
  pub first_name: String,
  #[validate(length(min = 1, max = 255))]
  pub last_name: String,
  #[validate(length(min = 1, max = 255), email)]
  pub email: String,
  #[validate(length(min = 1, max = 255))]
  pub phone: String,
  #[validate(length(min = 1, max = 255), custom = "validate_date_of_birth")]
  pub date_of_birth: String,
  #[validate(length(min = 1, max = 255))]
  pub region: String,
  #[validate(length(min = 1, max = 255))]
  pub register_number: String,
  #[validate(range(min = 0, max = 700))]
  pub toefl_score: i32,
  #[validate(range(min = 0, max = 9))]
  pub ielts_score: i32,
}

#[doc = "Validate date of birth"]
fn validate_date_of_birth(date: &str) -> Result<(), ValidationError> {
  let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

  if !date_regex.is_match(date) {
    return Err(ValidationError::new("Date format is not valid"));
  }

  let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d");

  if let Ok(date) = parsed_date {
    let current_date = Utc::now().naive_utc().date();

    if date <= current_date {
      return Ok(());
    }
  }

  Err(ValidationError::new("Date format is not valid"))
}