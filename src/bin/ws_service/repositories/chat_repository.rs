use chrono::Utc;
use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::{
  models::chat_model::*,
  structs::session_struct::ChatMessage
};

use scholarmate_api::helpers::parse::convert_vec_to_values;

#[doc = "Insert new chat"]
pub async fn insert_chat_data(pool: Pool<Postgres>, body: ChatMessage) -> Vec<Value> {
  let current_date_time = Utc::now().naive_utc().date();
  let data = sqlx::query_as!(ChatModel,
    "insert into chats (room_id, user_id, message, created_at) values ($1, $2, $3, $4) returning *",
    body.room_id, body.user_id, body.message, current_date_time)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Fetch chat data by room id"]
pub async fn fetch_chat_data_by_room_id(pool: Pool<Postgres>, room_id: i32) -> Vec<Value> {
  let data = sqlx::query_as!(ChatModel,
    "select * from chats where room_id = $1", room_id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}