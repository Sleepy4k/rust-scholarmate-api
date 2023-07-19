use chrono::prelude::*;

#[doc = "Check if number is positif"]
pub fn check_if_positif_number(number: i32) -> bool {
  number > 0
}

#[doc = "Check if date is valid"]
pub fn check_if_valid_date(date: &str) -> bool {
  let date = date.parse::<NaiveDate>();

  match date {
    Ok(result) => {
      let current_date = Utc::now().naive_utc().date();

      result <= current_date
    },
    Err(_) => false,
  }
}

#[doc = "Check if role is valid"]
pub fn check_if_valid_role(role: &str) -> bool {
  role == "admin" || role == "user"
}