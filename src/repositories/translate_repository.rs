use sqlx::{Pool, Postgres};

use crate::{
  models::translate_model::*,
  enums::error_enum::ErrorEnum
};

#[doc = "Get all translate column data."]
pub async fn get_translate_data(pool: Pool<Postgres>) -> anyhow::Result<Vec<TranslateModel>, ErrorEnum> {
  match sqlx::query_as!(TranslateModel, "select * from translate_columns")
    .fetch_all(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Find a translate column data by table name."]
pub async fn find_translate_data_by_table_name(pool: Pool<Postgres>, table: String) -> anyhow::Result<Vec<DetailTranslateModel>, ErrorEnum> {
  match sqlx::query_as!(DetailTranslateModel, "select column_name, language from translate_columns where table_name = $1", table)
    .fetch_optional(&pool)
    .await {
      Ok(data) => {
        match data {
          Some(data) => Ok(vec![data]),
          None => Err(ErrorEnum::CustomError(String::from("translate column not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}