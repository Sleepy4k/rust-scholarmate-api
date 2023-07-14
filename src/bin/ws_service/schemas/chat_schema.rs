use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralChatSchema {
  pub uid: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DetailChatSchema {
  pub name: String,
  pub sender: String,
  pub reciver: i32,
}