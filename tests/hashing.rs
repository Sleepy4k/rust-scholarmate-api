use pretty_assertions::{assert_eq, assert_ne};

use scholarmate_api::helpers::auth::*;

#[test]
#[doc(hidden)]
fn test_hash_password() {
  let data = "password";
  let hash = hash_password(data);
  let hash_again = hash_password(data);

  assert_eq!(hash, hash_again);
}

#[test]
#[doc(hidden)]
fn test_hash_password_wrong_password() {
  let data = "password";
  let hash = hash_password(data);
  let hash_again = hash_password(format!("{}123", data).as_str());

  assert_ne!(hash, hash_again);
}

#[test]
#[doc(hidden)]
fn test_verify_password() {
  let data = "password";
  let hashed_password = hash_password(data);
  let result = verify_password(data, hashed_password.as_str());

  assert_eq!(result, true);
}

#[test]
#[doc(hidden)]
fn test_verify_password_wrong_password() {
  let data = "password";
  let hashed_password = hash_password(data);
  let result = verify_password(format!("{}123", data).as_str(), hashed_password.as_str());

  assert_eq!(result, false);
}
