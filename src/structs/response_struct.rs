use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Debug)]
pub struct ResponseStruct {
  pub status: String,
  pub message: String,
  pub data: Vec<Value>,
}

#[derive(Serialize, Debug)]
pub struct ResponseCookieStruct {
  pub status: String,
  pub message: String,
  pub data: Vec<Value>,
  pub token: String,
}