use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::{
  models::auth_model::*,
  helpers::parse::convert_vec_to_values
};

#[doc = "Fetch all user data"]
pub async fn fetch_user_data(pool: Pool<Postgres>) -> Vec<Value> {
  let data = sqlx::query_as!(FilteredUserModel, "select id, email, role from users")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}
