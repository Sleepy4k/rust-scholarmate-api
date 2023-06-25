use validator::Validate;
use pretty_assertions::assert_eq;

#[derive(Validate)]
struct RequestEmailSchema {
  #[validate(email)]
  email: String
}

#[derive(Validate)]
struct RequestLengthSchema {
  #[validate(length(min = 1, max = 255))]
  name: String
}

#[derive(Validate)]
struct RequestUrlSchema {
  #[validate(url)]
  url: String
}

#[test]
#[doc(hidden)]
fn test_validate_email() {
  let request = RequestEmailSchema {
    email: String::from("johndoe123@example.com")
  };

  let result = request.validate();

  assert_eq!(result.is_ok(), true);
}

#[test]
#[doc(hidden)]
fn test_validate_email_error() {
  let request = RequestEmailSchema {
    email: String::from("johndoe123.com")
  };

  let result = request.validate();

  assert_eq!(result.is_err(), true);
}

#[test]
#[doc(hidden)]
fn test_validate_length() {
  let request = RequestLengthSchema {
    name: String::from("John Doe")
  };

  let result = request.validate();

  assert_eq!(result.is_ok(), true);
}

#[test]
#[doc(hidden)]
fn test_validate_length_error() {
  let request = RequestLengthSchema {
    name: String::new()
  };

  let result = request.validate();

  assert_eq!(result.is_err(), true);
}

#[test]
#[doc(hidden)]
fn test_validate_url() {
  let request = RequestUrlSchema {
    url: String::from("https://google.com")
  };

  let result = request.validate();

  assert_eq!(result.is_ok(), true);
}

#[test]
#[doc(hidden)]
fn test_validate_url_error() {
  let request = RequestUrlSchema {
    url: String::from("google.com")
  };

  let result = request.validate();

  assert_eq!(result.is_err(), true);
}
