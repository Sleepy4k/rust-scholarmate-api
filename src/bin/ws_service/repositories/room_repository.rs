use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

use crate::models::room_model::*;

use scholarmate_api::enums::error_enum::ErrorEnum;

#[doc = "Get all room data."]
pub async fn get_room_data(pool: Pool<Postgres>) -> anyhow::Result<Vec<RoomModel>, ErrorEnum> {
  match sqlx::query_as!(RoomModel, "select * from rooms")
    .fetch_all(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Create a room data."]
pub async fn create_room_data(pool: Pool<Postgres>, name: String, members: String, current_date_time: NaiveDate) -> anyhow::Result<RoomModel, ErrorEnum> {
  match sqlx::query_as!(RoomModel,
    "insert into rooms (name, members, updated_at, created_at) values ($1, $2, $3, $4) returning *",
    name, members, current_date_time, current_date_time)
    .fetch_one(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Find a room data by id."]
pub async fn find_room_data(pool: Pool<Postgres>, room_id: i32) -> anyhow::Result<RoomModel, ErrorEnum> {
  match sqlx::query_as!(RoomModel,
    "select * from rooms where id = $1", room_id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("room not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Delete room data by room id"]
pub async fn delete_room_data(pool: Pool<Postgres>, room_id: i32) -> anyhow::Result<RoomModel, ErrorEnum> {
  match sqlx::query_as!(RoomModel,
    "delete from rooms where id = $1 returning *", room_id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("room not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}