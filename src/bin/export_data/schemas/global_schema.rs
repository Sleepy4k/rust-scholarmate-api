use serde_json::Value;
use validator::Validate;
use serde::{Serialize, Deserialize};

#[doc = "General export schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct GeneralSchema {
  #[validate(required)]
 pub fields: Option<Value>
}