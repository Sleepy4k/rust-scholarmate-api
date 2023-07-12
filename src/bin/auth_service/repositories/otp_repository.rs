use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::models::otp_model::*;

use scholarmate_api::helpers::parse::convert_vec_to_values;

#[doc = "Fetch otp data by token"]
pub async fn fetch_otp_data_by_token(pool: Pool<Postgres>, token: String) -> Option<OTPModel> {
  let data = sqlx::query_as!(OTPModel, "select * from tokens where token = $1", token)
    .fetch_optional(&pool)
    .await
    .unwrap();

  data
}

#[doc = "Insert new token for otp"]
pub async fn insert_otp_data(pool: Pool<Postgres>, token: String, otp: String, email: String) -> anyhow::Result<bool> {
  match sqlx::query!(
    "insert into tokens (token, otp, email) values ($1, $2, $3)",
    token, otp, email)
    .execute(&pool)
    .await {
      Ok(_) => Ok(true),
      Err(_) => Ok(false)
    }
}

#[doc = "Delete otp data by token"]
pub async fn delete_otp_data(pool: Pool<Postgres>, token: String) -> Vec<Value> {
  let data = sqlx::query_as!(OTPModel,
    "delete from tokens where token = $1 returning *",
    token)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}