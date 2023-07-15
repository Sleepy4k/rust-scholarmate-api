use serde::{Serialize, Deserialize};

#[doc = "GeneralChatSchema is the struct that will be used to general chat schema in websocket."]
#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralChatSchema {
  pub uid: i32
}

#[doc = "DetailChatSchema is the struct that will be used to detail chat schema in websocket."]
#[derive(Debug, Deserialize, Serialize)]
pub struct DetailChatSchema {
  pub name: String,
  pub sender: String,
  pub reciver: i32,
}