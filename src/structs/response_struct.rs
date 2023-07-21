use serde_json::Value;
use serde::{Serialize, Deserialize};

#[doc = "Response struct"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStruct {
  pub status: String,
  pub message: String,
  pub data: Value,
}

#[doc = "Response struct with token"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStructWithToken {
  pub status: String,
  pub message: String,
  pub token: String,
  pub data: Value,
}