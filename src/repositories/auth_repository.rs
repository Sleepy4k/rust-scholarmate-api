use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::{
  models::auth_model::*,
  schemas::auth_schema::*,
  helpers::parse::convert_vec_to_values
};

#[doc = "Fetch all user data"]
pub async fn fetch_user_data(pool: Pool<Postgres>) -> Vec<Value> {
  let data = sqlx::query_as!(UserModel, "select id, email, role from users")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Insert new user"]
pub async fn insert_user_data(pool: Pool<Postgres>, body: RegisterSchema) -> Vec<Value> {
  let data = sqlx::query_as!(UserModel,
    "insert into users (email, password, role) values ($1, $2, $3) returning id, email, role",
    body.email, body.password, body.role)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}
