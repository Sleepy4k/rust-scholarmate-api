use serde_json::Value;
use serde::{Serialize, Deserialize};

#[doc = "General export schema"]
#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralSchema {
  pub fields: Value
}