use serde::{Serialize, Deserialize};

#[doc = "Token struct"]
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenStruct {
  pub id: i32,
  pub role: String,
  pub email: String,
  pub iat: u64,
  pub exp: u64,
}
