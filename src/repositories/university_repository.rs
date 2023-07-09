use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::{
  models::university_model::*,
  schemas::university_schema::*,
  helpers::parse::convert_vec_to_values
};

#[doc = "Fetch all university data"]
pub async fn fetch_university_data(pool: Pool<Postgres>) -> Vec<Value> {
  let data = sqlx::query_as!(UniversityModel, "select * from universities")
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Insert new university"]
pub async fn insert_university_data(pool: Pool<Postgres>, body: UniversitySchema) -> Vec<Value> {
  let data = sqlx::query_as!(UniversityModel,
    "insert into universities (name, major, quantity, description, image, link, alias) values ($1, $2, $3, $4, $5, $6, $7) returning *",
    body.name, body.major, body.quantity, body.description, body.image, body.link, body.alias)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Fetch university data by id"]
pub async fn fetch_university_data_by_id(pool: Pool<Postgres>, id: i32) -> Option<UniversityModel> {
  let data = sqlx::query_as!(UniversityModel, "select * from universities where id = $1", id)
    .fetch_optional(&pool)
    .await
    .unwrap();

    data
}

#[doc = "Fetch university data by exists column"]
pub async fn fetch_university_data_by_exists_column(pool: Pool<Postgres>, name: String, major: String) -> Option<UniversityModel> {
  let data = sqlx::query_as!(UniversityModel, "select * from universities where name = $1 and major = $2", name, major)
    .fetch_optional(&pool)
    .await
    .unwrap();
  
  data
}

#[doc = "Update university data"]
pub async fn update_university_data(pool: Pool<Postgres>, id: i32, body: UniversitySchema) -> Vec<Value> {
  let data = sqlx::query_as!(UniversityModel,
    "update universities set name = $1, major = $2, quantity = $3, description = $4, image = $5, link = $6, alias = $7 where id = $8 returning *",
    body.name, body.major, body.quantity, body.description, body.image, body.link, body.alias, id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}

#[doc = "Delete university data"]
pub async fn delete_university_data(pool: Pool<Postgres>, id: i32) -> Vec<Value> {
  let data = sqlx::query_as!(UniversityModel,
    "delete from universities where id = $1 returning *", id)
    .fetch_all(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  result
}
