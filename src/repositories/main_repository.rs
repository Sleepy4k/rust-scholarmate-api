use sqlx::{Pool, Postgres, postgres::PgRow};

#[doc = "Fetch data"]
pub async fn fetch_data(pool: Pool<Postgres>, query_str: &str) -> Vec<PgRow> {
  let query_string = format!("{}", query_str);
  let data = sqlx::query(query_string.as_str())
    .fetch_all(&pool)
    .await
    .unwrap();

  data
}

#[doc = "Check data"]
pub async fn check_data(pool: Pool<Postgres>, query_str: &str) -> bool {
  let query_string = format!("select exists({}) as data_exists", query_str);
  let query = sqlx::query_scalar::<_, bool>(query_string.as_str())
    .fetch_one(&pool)
    .await
    .unwrap();

  query
}
