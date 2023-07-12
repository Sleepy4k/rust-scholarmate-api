use pretty_assertions::{assert_eq, assert_ne};

use scholarmate_api::helpers::{
  hashing::*,
  parse::get_email_parts
};

#[test]
#[doc(hidden)]
fn test_hash_password() {
  let data = "password";
  let salt = get_email_parts("johndoe123@gmail.com")[0];
  let hash = hash_password(data, salt.as_bytes());
  let hash_again = hash_password(data, salt.as_bytes());

  assert_eq!(hash, hash_again);
}

#[test]
#[doc(hidden)]
fn test_hash_password_wrong_password() {
  let data = "password";
  let salt = get_email_parts("johndoe123@gmail.com")[0];
  let hash = hash_password(data, salt.as_bytes());
  let hash_again = hash_password(format!("{}123", data).as_str(), salt.as_bytes());

  assert_ne!(hash, hash_again);
}

#[test]
#[doc(hidden)]
fn test_verify_password() {
  let data = "password";
  let salt = get_email_parts("johndoe123@gmail.com")[0];
  let hashed_password = hash_password(data, salt.as_bytes());
  let result = verify_password(data, hashed_password.as_str());

  assert_eq!(result, true);
}

#[test]
#[doc(hidden)]
fn test_verify_password_wrong_password() {
  let data = "password";
  let salt = get_email_parts("johndoe123@gmail.com")[0];
  let hashed_password = hash_password(data, salt.as_bytes());
  let result = verify_password(format!("{}123", data).as_str(), hashed_password.as_str());

  assert_eq!(result, false);
}
