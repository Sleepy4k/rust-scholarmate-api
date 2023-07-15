use chrono::NaiveDate;
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[doc = "RoomModel is the struct that will be used to room model in websocket."]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomModel {
  pub id: i32,
  pub name: String,
  pub members: String,
  pub updated_at: NaiveDate,
  pub created_at: NaiveDate,
}

#[doc = "NewRoomModel is the struct that will be used to new room model in websocket."]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRoomModel {
  pub name: String,
  pub members: String,
}

#[doc = "RoomResponseModel is the struct that will be used to room response model in websocket."]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomResponseModel {
  pub room: RoomModel,
  pub users: Vec<Value>,
}