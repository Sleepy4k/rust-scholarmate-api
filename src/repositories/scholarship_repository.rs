use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::{
  models::scholarship_model::*,
  schemas::scholarship_schema::*,
  helpers::parse::convert_vec_to_values
};

#[doc = "Fetch all scholarship data"]
pub async fn fetch_scholarship_data(pool: Pool<Postgres>) -> Vec<Value> {
  let data = sqlx::query_as!(DetailScholarshipModel,
    "select s.id, s.name, s.description, s.quantity, s.requirement, u.id as univ_id,
      u.name as univ_name, u.alias as univ_alias, u.description as univ_description,
      u.major as univ_major from scholarships s join universities u on s.univ_id = u.id")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Insert new scholarship"]
pub async fn insert_scholarship_data(pool: Pool<Postgres>, body: ScholarshipSchema) -> Vec<Value> {
  let data = sqlx::query_as!(ScholarshipModel,
    "insert into scholarships (name, description, quantity, requirement, univ_id) values ($1, $2, $3, $4, $5) returning *",
      body.name, body.description, body.quantity, body.requirement, body.univ_id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Fetch scholarship data by id"]
pub async fn fetch_scholarship_data_by_id(pool: Pool<Postgres>, id: i32) -> Option<ScholarshipModel> {
  let data = sqlx::query_as!(ScholarshipModel, "select * from scholarships where id = $1", id)
    .fetch_optional(&pool)
    .await
    .unwrap();

    data
}

#[doc = "Update scholarship data"]
pub async fn update_scholarship_data(pool: Pool<Postgres>, id: i32, body: ScholarshipSchema) -> Vec<Value> {
  let data = sqlx::query_as!(ScholarshipModel,
    "update scholarships set name = $1, description = $2, quantity = $3, requirement = $4, univ_id = $5 where id = $6 returning *",
      body.name, body.description, body.quantity, body.requirement, body.univ_id, id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Delete scholarship data"]
pub async fn delete_scholarship_data(pool: Pool<Postgres>, id: i32) -> Vec<Value> {
  let data = sqlx::query_as!(ScholarshipModel, "delete from scholarships where id = $1 returning *", id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}
