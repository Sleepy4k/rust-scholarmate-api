use serde_json::Value;
use sqlx::{Pool, Postgres, postgres::PgRow};

use crate::{
  models::application_model::*,
  schemas::application_schema::*,
  helpers::parse::convert_vec_to_values
};

#[doc = "Fetch all application data"]
pub async fn fetch_application_data(pool: Pool<Postgres>) -> Vec<Value> {
  let data = sqlx::query_as!(ApplicationModel, "select * from applications")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Fetch application data by id"]
pub async fn fetch_application_data_by_id(pool: Pool<Postgres>, id: i32) -> Vec<Value> {
  let data = sqlx::query_as!(DetailApplicationModel,
    "select app.id, univ.id as univ_id, univ.name, univ.major, univ.image, app.status from applications app
      join universities univ on app.univ_id = univ.id where student_id = $1
      order by univ.id desc"
    , id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Insert new application"]
pub async fn insert_application_data(pool: Pool<Postgres>, body: ApplicationSchema) -> Vec<Value> {
  let data = sqlx::query_as!(ApplicationModel,
    "insert into applications (univ_id, student_id, status, major) values ($1, $2, $3, $4) returning *",
      body.univ_id, body.student_id, body.status, body.major)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Update application data"]
pub async fn update_application_data(pool: Pool<Postgres>, id: i32) -> Vec<PgRow> {
  let data = sqlx::query!(
    "with updated_university as (update universities set quantity = quantity - 1 where id = $1 returning id)
    update scholarships set quantity = quantity - 1 where univ_id in (select id from updated_university)",
    id)
    .fetch_all(&pool)
    .await
    .unwrap();

    data
}
