use chrono::Utc;
use serde_json::Value;
use sqlx::{Pool, Postgres};

use std::collections::{HashMap, HashSet};

use crate::models::room_model::*;

use scholarmate_api::{
  helpers::parse::convert_vec_to_values,
  repositories::auth_repository::fetch_user_data,
};

#[doc = "Insert new room"]
pub async fn insert_room_data(pool: Pool<Postgres>, name: String, members: String) -> Vec<Value> {
  let current_date_time = Utc::now().naive_utc().date();
  let data = sqlx::query_as!(RoomModel,
    "insert into rooms (name, members, updated_at, created_at) values ($1, $2, $3, $4) returning *",
    name, members, current_date_time, current_date_time)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Fetch room data by user id"]
pub async fn fetch_room_data_by_user_id(pool: Pool<Postgres>, id: i32) -> Vec<Value> {
  let mut rooms_data = sqlx::query_as!(RoomModel,
    "select * from rooms")
    .fetch_all(&pool)
    .await
    .unwrap();

  rooms_data = rooms_data
    .into_iter()
    .filter(|item| item.members.contains(&id.to_string()))
    .collect::<Vec<_>>();

  let mut ids = HashSet::new();
  let mut rooms_map = HashMap::new();
  let data = rooms_data.to_vec();

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

  let users_map: HashMap<String, Value> = HashMap::from_iter(
    filtered_users_data
      .into_iter()
      .map(|item| (item["id"].to_string(), item)),
  );

  let response_rooms = rooms_data.into_iter().map(|room| {
    let users = rooms_map
      .get(&room.id.to_string())
      .unwrap()
      .into_iter()
      .map(|id| users_map.get(id.to_owned()).unwrap().clone())
      .collect::<Vec<_>>();
    return RoomResponseModel{ room, users };
  }).collect::<Vec<_>>();

  let result = convert_vec_to_values(response_rooms);

  result
}

#[doc = "Fetch room data by room id"]
pub async fn fetch_room_data_by_room_id(pool: Pool<Postgres>, room_id: i32) -> Option<RoomModel> {
  let data = sqlx::query_as!(RoomModel,
    "select * from rooms where id = $1", room_id)
    .fetch_optional(&pool)
    .await
    .unwrap();

  data
}

pub async fn delete_room_data(pool: Pool<Postgres>, room_id: i32) -> Vec<Value> {
  let data = sqlx::query_as!(RoomModel,
    "delete from rooms where id = $1 returning *", room_id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}