use serde::Serialize;
use serde_merge::Map;
use serde_json::{Value, to_value};
use chrono::{Local, NaiveDateTime, format::strftime::StrftimeItems};

#[doc = "Convert string to slug"]
pub fn slugify(text: &str, replacer: &str) -> String {
  text.replace(" ", replacer)
}

#[doc = "function to convert value to string"]
pub fn to_str(data: Value) -> String {
  data.as_str().unwrap_or("").to_owned()
}

#[doc = "function to convert value to i64"]
pub fn to_i64(data: Value) -> i64 {
  data.as_i64().unwrap_or(0) as i64
}

#[doc = "function to convert value to i32"]
pub fn to_i32(data: Value) -> i32 {
  data.as_i64().unwrap_or(0) as i32
}

#[doc = "function to convert value to f64"]
pub fn to_f64(map: Value) -> f64 {
  map.as_f64().unwrap_or(0.0).to_owned()
}

#[doc = "function to convert value to array"]
pub fn to_array(map: Value) -> Vec<Value> {
  map.as_array().unwrap_or(&Vec::new()).to_owned()
}

#[doc = "function to convert any struct to json"]
pub fn convert_vec_to_values<T: Serialize>(data: Vec<T>) -> Vec<Value> {
  data.into_iter().map(|item| to_value(item).unwrap()).collect()
}

#[doc = "Get email parts"]
pub fn get_email_parts(email: &str) -> Vec<&str> {
  let email_parts = email.split("@").collect::<Vec<&str>>();

  email_parts
}

#[doc = "Get modified duration"]
pub fn modified_duration(start_time: i64, end_time: i64) -> String {
  let now = Local::now().timestamp();
  let delta = if end_time <= 0 { now - start_time } else { end_time - start_time };
  let dt = NaiveDateTime::from_timestamp_opt(delta,0).unwrap();
  let formatted_date = dt.format_with_items(StrftimeItems::new("H: %H, M: %M, S: %S"));

  formatted_date.to_string()
}

#[doc = "function to get value from map"]
pub fn map_get(key: &str, data: Value) -> Value {
  data
    .as_object()
    .unwrap_or(&Map::new())
    .get(key)
    .unwrap_or(&Value::Null)
    .to_owned()
}

#[doc = "function to convert vector of string to string"]
pub fn vec_to_string(vec: Vec<&str>) -> String {
  let mut res = String::new();

  for (i, el) in vec.into_iter().enumerate() {
    if i != 0 {res.push_str(",")}
    
    res.push_str("'");
    res.push_str(el);
    res.push_str("'");
  }

  res
}
