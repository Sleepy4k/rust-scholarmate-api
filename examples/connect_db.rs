use dotenv::dotenv;
use serde_json::Value;
use sqlx::{Pool, Postgres};

use scholarmate_api::{
  models::auth_model::FilteredUserModel,
  helpers::{
    database::connect_postgres,
    parse::convert_vec_to_values
  }
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();

  let pool = connect_postgres().await;

  let data = fetch_query_data(pool).await;

  println!("{:?}", data);

  Ok(())
}

#[doc = "Fetch query data"]
async fn fetch_query_data(pool: Pool<Postgres>) -> Vec<Value> {
  let data = sqlx::query_as!(FilteredUserModel, "select id, email, role from users")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}
