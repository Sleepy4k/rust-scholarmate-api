use chrono::Utc;
use serde_json::Value;
use sqlx::{Pool, Postgres};
use std::collections::{HashMap, HashSet};

use crate::{
  models::chat_model::*,
  models::room_model::RoomModel,
  structs::session_struct::ChatMessage
};

use scholarmate_api::{
  helpers::parse::convert_vec_to_values,
  repositories::auth_repository::fetch_user_data,
};

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
  let rooms_data = sqlx::query_as!(RoomModel,
    "select * from rooms where id = $1", room_id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let mut ids = HashSet::new();
  let mut rooms_map = HashMap::new();
  let data = rooms_data.to_owned();

  for room in &data {
    let user_ids = room
      .members
      .split(",")
      .into_iter()
      .collect::<Vec<_>>();

    for id in user_ids.to_vec() {
      ids.insert(id.to_string());
    }

    rooms_map.insert(room.id.to_string(), user_ids.to_vec());
  }

  let ids = ids.into_iter().collect::<Vec<_>>();
  let users_data = fetch_user_data(pool.clone()).await;
  let filtered_users_data = users_data
    .into_iter()
    .filter(|item| ids.contains(&item["id"].to_string()))
    .collect::<Vec<_>>();

  let chats_data = sqlx::query_as!(ChatModel,
    "select * from chats where room_id = $1", room_id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let converted_chats = convert_vec_to_values(chats_data);

  let users_map: HashMap<String, Value> = HashMap::from_iter(
    filtered_users_data
      .into_iter()
      .map(|item| (item["id"].to_string(), item)),
  );

  let chats_map: HashMap<String, Value> = HashMap::from_iter(
    converted_chats
      .into_iter()
      .map(|item| (format!("{}-{}", item["room_id"].to_string(), item["id"].to_string()), item)),
  );

  let response_chats = rooms_data.into_iter().map(|room| {
    let users = rooms_map
      .get(&room.id.to_string())
      .unwrap()
      .into_iter()
      .map(|id| users_map.get(id.to_owned()).unwrap().clone())
      .collect::<Vec<_>>();
   
    let chats = rooms_map
      .get(&room.id.to_string())
      .unwrap()
      .into_iter()
      .map(|id| {
        let chat = chats_map.get(&format!("{}-{}", room.id.to_string(), id.to_string()));

        match chat {
          Some(chat) => chat.clone(),
          None => Value::Null
        }
      })
      .collect::<Vec<_>>();
    return ChatResponseModel{ room, users, chats };
  }).collect::<Vec<_>>();

  let result = convert_vec_to_values(response_chats);

  result
}