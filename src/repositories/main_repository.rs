use sqlx::{Pool, Postgres};

use crate::enums::error_enum::ErrorEnum;

#[doc = "Check data exist or not"]
pub async fn check_data(pool: Pool<Postgres>, query_str: &str) -> anyhow::Result<bool, ErrorEnum> {
  let query_string = format!("select exists({}) as data_exists", query_str);

  match sqlx::query_scalar::<_, bool>(query_string.as_str())
    .fetch_one(&pool)
    .await
  {
    Ok(query) => Ok(query),
    Err(_) => Err(ErrorEnum::InternalServerError),
  }
}
