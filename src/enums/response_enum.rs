use serde_json::Value;
use serde::{Serialize, Deserialize};

#[doc = "Response enum for data"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseDataEnum {
  SingleValue(Value),
  ArrayValue(Vec<Value>),
  ArrayUsize(Vec<usize>),
}

impl ResponseDataEnum {
  #[doc = "Get value from enum"]
  pub fn get_value(&self) -> Value {
    match self {
      ResponseDataEnum::SingleValue(value) => value.clone(),
      ResponseDataEnum::ArrayValue(value) => Value::Array(value.clone()),
      ResponseDataEnum::ArrayUsize(value) => Value::Array(value.iter().map(|x| Value::Number(serde_json::Number::from_f64(*x as f64).unwrap())).collect())
    }
  }

  #[doc = "Get array from enum"]
  pub fn get_array(&self) -> Vec<Value> {
    match self {
      ResponseDataEnum::SingleValue(value) => vec![value.clone()],
      ResponseDataEnum::ArrayValue(value) => value.clone(),
      ResponseDataEnum::ArrayUsize(value) => value.iter().map(|x| Value::Number(serde_json::Number::from_f64(*x as f64).unwrap())).collect()
    }
  }

  #[doc = "Check if value is empty"]
  pub fn is_empty(&self) -> bool {
    match self {
      ResponseDataEnum::SingleValue(value) => value.is_null(),
      ResponseDataEnum::ArrayValue(value) => value.is_empty(),
      ResponseDataEnum::ArrayUsize(value) => value.is_empty()
    }
  }
}