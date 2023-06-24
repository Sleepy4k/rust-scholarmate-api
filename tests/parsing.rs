use chrono::NaiveDate;
use serde_json::{json, Value};
use pretty_assertions::assert_eq;

use scholarmate_api::{helpers::parse::*, models::student_model::*};

#[test]
#[doc(hidden)]
fn test_convert_vec_to_string() {
  let data = vec![
    "student001",
    "student002",
    "student003",
  ];

  let result = vec_to_string(data);

  assert_eq!(result, String::from("'student001','student002','student003'"));
}

#[test]
#[doc(hidden)]
fn test_convert_vec_to_values() {
  let data = vec![
    StudentModel {
      id: 1,
      first_name: String::from("John"),
      last_name: String::from("Doe"),
      email: String::from("johndoe123@gmail.com"),	
      phone: String::from("08123456789"),
      date_of_birth: NaiveDate::parse_from_str("1999-01-01", "%Y-%m-%d").unwrap(),
      region: String::from("Indonesia"),
      register_number: String::from("123456789"),
      toefl_score: 500,
      ielts_score: 8,
    }
  ];

  let result = convert_vec_to_values(data);

  assert_eq!(result, vec![
    json!({
      "id": 1,
      "first_name": "John",
      "last_name": "Doe",
      "email": "johndoe123@gmail.com",
      "phone": "08123456789",
      "date_of_birth": "1999-01-01",
      "region": "Indonesia",
      "register_number": "123456789",
      "toefl_score": 500,
      "ielts_score": 8,
    })
  ]);
}

#[test]
#[doc(hidden)]
fn test_string_to_slug() {
  let data = "John Doe";
  let result = slugify(data);

  assert_eq!(result, "John-Doe");
}

#[test]
#[doc(hidden)]
fn test_map_get_data() {
  let data = json!({
    "id": 1,
    "first_name": "John",
    "last_name": "Doe",
    "email": "johndoe123@gmail.com",
    "phone": "08123456789",
    "date_of_birth": "1999-01-01",
    "region": "Indonesia",
    "register_number": "123456789",
    "toefl_score": 500,
    "ielts_score": 8,
  });

  let result = map_get("email", data);

  assert_eq!(result, Value::String(String::from("johndoe123@gmail.com")));
}

#[test]
#[doc(hidden)]
fn test_modified_duration() {
  let start_time = 0;
  let end_time = 45;

  let result = modified_duration(start_time, end_time);

  assert_eq!(result, String::from("H: 00, M: 00, S: 45"));
}
