use chrono::NaiveDate;
use serde_json::Value;
use serde::{Deserialize, Serialize};

use crate::models::room_model::RoomModel;

#[doc = "ChatModel is the struct that will be used to chat model in websocket."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatModel {
  pub id: i32,
  pub room_id: i32,
  pub user_id: i32,
  pub message: String,
  pub created_at: NaiveDate,
}

#[doc = "NewChatModel is the struct that will be used to new chat model in websocket."]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewChatModel {
  pub user_id: i32,
  pub room_id: i32,
  pub message: String,
}

#[doc = "ChatResponseModel is the struct that will be used to chat response model in websocket."]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponseModel {
  pub chats: Vec<Value>,
  pub room: RoomModel,
  pub users: Vec<Value>,
}