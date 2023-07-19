use sqlx::{Pool, Postgres};

use crate::{
  models::scholarship_model::*,
  enums::error_enum::ErrorEnum,
  schemas::scholarship_schema::ScholarshipSchema
};

#[doc = "Get all scholarship data."]
pub async fn get_scholarship_data(pool: Pool<Postgres>) -> anyhow::Result<Vec<DetailScholarshipModel>, ErrorEnum> {
  match sqlx::query_as!(DetailScholarshipModel,
    "select s.id, s.name, s.description, s.quantity, s.requirement, u.id as univ_id,
      u.name as univ_name, u.alias as univ_alias, u.description as univ_description,
      u.major as univ_major from scholarships s join universities u on s.univ_id = u.id")
    .fetch_all(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Create an scholarship data."]
pub async fn create_scholarship_data(pool: Pool<Postgres>, body: ScholarshipSchema) -> anyhow::Result<ScholarshipModel, ErrorEnum> {
  match sqlx::query_as!(ScholarshipModel,
    "insert into scholarships (name, description, quantity, requirement, univ_id) values ($1, $2, $3, $4, $5) returning *",
      body.name, body.description, body.quantity, body.requirement, body.univ_id)
    .fetch_one(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Find an scholarship data by id."]
pub async fn find_scholarship_data(pool: Pool<Postgres>, id: i32) -> anyhow::Result<ScholarshipModel, ErrorEnum> {
  match sqlx::query_as!(ScholarshipModel, "select * from scholarships where id = $1 limit 1", id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("scholarship not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Update an existing scholarship data."]
pub async fn update_scholarship_data(pool: Pool<Postgres>, id: i32, body: ScholarshipSchema) -> anyhow::Result<ScholarshipModel, ErrorEnum> {
  match sqlx::query_as!(ScholarshipModel,
    "update scholarships set name = $1, description = $2, quantity = $3, requirement = $4, univ_id = $5 where id = $6 returning *",
      body.name, body.description, body.quantity, body.requirement, body.univ_id, id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("scholarship not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Delete scholarship data by id."]
pub async fn delete_scholarship_data(pool: Pool<Postgres>, id: i32) -> anyhow::Result<ScholarshipModel, ErrorEnum> {
  match sqlx::query_as!(ScholarshipModel, "delete from scholarships where id = $1 returning *", id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("scholarship not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}
