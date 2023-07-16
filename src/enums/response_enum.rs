use serde_json::Value;
use serde::{Serialize, Deserialize};

#[doc = "Response enum for data"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseDataEnum<T> {
  SingleValue(Value),
  ArrayValue(Vec<Value>),
  SingleObject(T),
  ArrayObject(Vec<T>),
  ArrayUsize(Vec<usize>),
}

impl <T: Serialize> ResponseDataEnum<T> {
  pub fn get_value(&self) -> Value {
    match self {
      ResponseDataEnum::SingleValue(value) => value.clone(),
      ResponseDataEnum::ArrayValue(value) => Value::Array(value.clone()),
      ResponseDataEnum::SingleObject(value) => serde_json::to_value(value).unwrap(),
      ResponseDataEnum::ArrayObject(value) => serde_json::to_value(value).unwrap(),
      ResponseDataEnum::ArrayUsize(value) => Value::Array(value.iter().map(|x| Value::Number(serde_json::Number::from_f64(*x as f64).unwrap())).collect()),
    }
  }
}