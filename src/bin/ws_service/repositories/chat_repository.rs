use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

use crate::{
  models::chat_model::*,
  structs::session_struct::ChatMessage
};

use scholarmate_api::enums::error_enum::ErrorEnum;

#[doc = "Create a chat data."]
pub async fn create_chat_data(pool: Pool<Postgres>, body: ChatMessage, current_date_time: NaiveDate) -> anyhow::Result<ChatModel, ErrorEnum> {
  match sqlx::query_as!(ChatModel,
    "insert into chats (room_id, user_id, message, created_at) values ($1, $2, $3, $4) returning *",
    body.room_id, body.user_id, body.message, current_date_time)
    .fetch_one(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Find a chat data by id."]
pub async fn find_chat_data(pool: Pool<Postgres>, room_id: i32) -> anyhow::Result<ChatModel, ErrorEnum> {
  match sqlx::query_as!(ChatModel,
    "select * from chats where room_id = $1", room_id)
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