use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::{
  models::forum_model::*,
  helpers::parse::convert_vec_to_values
};

#[doc = "Fetch all forum data"]
pub async fn fetch_forum_data(pool: Pool<Postgres>, id: i32) -> Vec<Value> {
  let data = sqlx::query_as!(ForumModel,
    "select universities.alias, forums.message, universities.image, universities.link from forums
      join universities on forums.univ_id = universities.id where student_id = $1
      order by forums.id desc"
    , id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}
