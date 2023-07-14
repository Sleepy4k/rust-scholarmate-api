use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatModel {
  pub id: i32,
  pub room_id: i32,
  pub user_id: i32,
  pub message: String,
  pub created_at: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewChatModel {
  pub user_id: i32,
  pub room_id: i32,
  pub message: String,
}