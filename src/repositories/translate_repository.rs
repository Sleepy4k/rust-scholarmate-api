use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::{
  models::translate_model::*,
  helpers::parse::convert_vec_to_values
};

#[doc = "function to fetch translate data"]
pub async fn fetch_translate_data(pool: Pool<Postgres>) -> Vec<Value> {
  let data = sqlx::query_as!(TranslateModel, "select * from translate_columns")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "function to fetch translate data with specific table"]
pub async fn fetch_translate_data_by_table(pool: Pool<Postgres>, table: String) -> Vec<Value> {
  let data = sqlx::query_as!(DetailTranslateModel, "select column_name, language from translate_columns where table_name = $1", table)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}