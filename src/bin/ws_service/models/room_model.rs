use chrono::NaiveDate;
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomModel {
  pub id: i32,
  pub name: String,
  pub members: String,
  pub updated_at: NaiveDate,
  pub created_at: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRoomModel {
  pub name: String,
  pub members: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomResponseModel {
  pub room: RoomModel,
  pub users: Vec<Value>,
}