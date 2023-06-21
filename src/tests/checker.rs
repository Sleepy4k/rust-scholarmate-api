use pretty_assertions::assert_eq;

use crate::helpers::validation::*;

#[test]
fn test_check_data() {
  let data = "John Doe";
  let result = check_if_empty(data);

  assert_eq!(result, false);
}

#[test]
fn test_check_data_empty() {
  let data = "";
  let result = check_if_empty(data);

  assert_eq!(result, true);
}

#[test]
fn test_check_type() {
  let data = "John Doe";
  let result = check_type(data);

  assert_eq!(result, "&str");
}

#[test]
fn test_check_type_string() {
  let data = String::from("John Doe");
  let result = check_type(data);

  assert_eq!(result, "alloc::string::String");
}

#[test]
fn test_check_type_vec() {
  let data = vec!["John Doe"];
  let result = check_type(data);

  assert_eq!(result, "alloc::vec::Vec<&str>");
}

#[test]
fn test_check_type_i32() {
  let data = 1;
  let result = check_type(data);

  assert_eq!(result, "i32");
}

#[test]
fn test_check_type_void() {
  let data = ();
  let result = check_type(data);

  assert_eq!(result, "()");
}
