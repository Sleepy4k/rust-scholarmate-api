use pretty_assertions::{assert_eq, assert_ne};

use scholarmate_api::helpers::validation::*;

#[test]
#[doc(hidden)]
fn test_check_if_positif_number() {
  let data = 1;
  let result = check_if_positif_number(data);

  assert_eq!(result, true);
}

#[test]
#[doc(hidden)]
fn test_check_if_positif_number_zero() {
  let data = 0;
  let result = check_if_positif_number(data);

  assert_eq!(result, false);
}

#[test]
#[doc(hidden)]
fn test_check_if_positif_number_negative() {
  let data = -1;
  let result = check_if_positif_number(data);

  assert_ne!(result, true);
}

#[test]
#[doc(hidden)]
fn test_check_if_date_valid() {
  let data = "2021-01-01";
  let result = check_if_valid_date(data);

  assert_eq!(result, true);
}

#[test]
#[doc(hidden)]
fn test_check_if_date_valid_wrong_format() {
  let data = "2021-01-01 00:00:00";
  let result = check_if_valid_date(data);

  assert_ne!(result, true);
}

#[test]
#[doc(hidden)]
fn test_check_if_date_valid_wrong_date() {
  let data = "2021-01-32";
  let result = check_if_valid_date(data);

  assert_ne!(result, true);
}

#[test]
#[doc(hidden)]
fn test_check_if_role_valid() {
  let data = "admin";
  let result = check_if_valid_role(data);

  assert_eq!(result, true);
}

#[test]
#[doc(hidden)]
fn test_check_if_role_valid_wrong_role() {
  let data = "admin123";
  let result = check_if_valid_role(data);

  assert_ne!(result, true);
}
