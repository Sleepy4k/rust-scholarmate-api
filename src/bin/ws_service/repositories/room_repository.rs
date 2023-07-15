use chrono::Utc;
use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::models::room_model::*;

use scholarmate_api::helpers::parse::convert_vec_to_values;

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

  let data = rooms_data.to_vec();
  let result = convert_vec_to_values(data);

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

#[doc = "Delete room data by room id"]
pub async fn delete_room_data(pool: Pool<Postgres>, room_id: i32) -> Vec<Value> {
  let data = sqlx::query_as!(RoomModel,
    "delete from rooms where id = $1 returning *", room_id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}