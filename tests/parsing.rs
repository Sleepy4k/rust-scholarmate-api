use serde::Serialize;
use serde_json::{json, Value};
use pretty_assertions::{assert_eq, assert_ne};

use scholarmate_api::helpers::parse::*;

#[derive(Serialize)]
pub struct UserModel {
  uid: i32,
  username: String,
  email: String,
}

#[test]
#[doc(hidden)]
fn test_convert_vec_to_string() {
  let data = vec![
    "student001",
    "student002",
    "student003",
  ];

  let convert = vec_to_string(data.to_owned());
  let convert_again = vec_to_string(data);

  assert_eq!(convert, convert_again);
}

#[test]
#[doc(hidden)]
fn test_convert_vec_to_string_wrong_data() {
  let mut data = vec![
    "student001",
    "student002",
    "student003",
  ];

  let convert = vec_to_string(data.to_owned());
  data.push("student004");
  let convert_again = vec_to_string(data);

  assert_ne!(convert, convert_again);
}

#[test]
#[doc(hidden)]
fn test_convert_vec_to_values() {
  let mock_data = Value::from(json!({
    "uid": 1,
    "username": "student001",
    "email": "johndoe123@example.com",
  }));

  let data = vec![
    UserModel {
      uid: mock_data["uid"].as_i64().unwrap() as i32,
      username: mock_data["username"].as_str().unwrap().to_string(),
      email: mock_data["email"].as_str().unwrap().to_string(),
    },
  ];

  let result = convert_vec_to_values(data);

  assert_eq!(result, vec![mock_data]);
}

#[test]
#[doc(hidden)]
fn test_convert_vec_to_values_wrong_data() {
  let mut mock_data = Value::from(json!({
    "uid": 1,
    "username": "student001",
    "email": "johndoe123@example.com",
  }));

  let data = vec![
    UserModel {
      uid: mock_data["uid"].as_i64().unwrap() as i32,
      username: mock_data["username"].as_str().unwrap().to_string(),
      email: mock_data["email"].as_str().unwrap().to_string(),
    },
  ];

  let result = convert_vec_to_values(data);

  mock_data["uid"] = json!(2);

  assert_ne!(result, vec![mock_data]);
}

#[test]
#[doc(hidden)]
fn test_string_to_slug() {
  let data = "John Doe";
  let result = slugify(data, "-");

  assert_eq!(result, "John-Doe");
}

#[test]
#[doc(hidden)]
fn test_map_get_data() {
  let data = json!({
    "uid": 1,
    "username": "student001",
    "email": "johndoe123@example.com",
  });

  let result = map_get("email", data.to_owned());

  assert_eq!(result, data["email"]);
}

#[test]
#[doc(hidden)]
fn test_map_get_data_wrong_key() {
  let data = json!({
    "uid": 1,
    "username": "student001",
    "email": "johndoe123@example.com",
  });

  let result = map_get("email", data.to_owned());

  assert_ne!(result, data["username"]);
}

#[test]
#[doc(hidden)]
fn test_modified_duration() {
  let start_time = 0;
  let end_time = 45;

  let result = modified_duration(start_time, end_time);

  assert_eq!(result, String::from("H: 00, M: 00, S: 45"));
}
