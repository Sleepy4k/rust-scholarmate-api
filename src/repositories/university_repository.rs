use sqlx::{Pool, Postgres};

use crate::{
  models::university_model::*,
  enums::error_enum::ErrorEnum,
  schemas::university_schema::UniversitySchema
};

#[doc = "Get all university data."]
pub async fn get_university_data(pool: Pool<Postgres>) -> anyhow::Result<Vec<UniversityModel>, ErrorEnum> {
  match sqlx::query_as!(UniversityModel, "select * from universities")
    .fetch_all(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Create an university data."]
pub async fn create_university_data(pool: Pool<Postgres>, body: UniversitySchema) -> anyhow::Result<UniversityModel, ErrorEnum> {
  match sqlx::query_as!(UniversityModel,
    "insert into universities (name, major, quantity, description, image, link, alias)
    values ($1, $2, $3, $4, $5, $6, $7) returning *", body.name, body.major,
    body.quantity, body.description, body.image, body.link, body.alias)
    .fetch_one(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Find an university data by id."]
pub async fn find_university_data(pool: Pool<Postgres>, id: i32) -> anyhow::Result<UniversityModel, ErrorEnum> {
  match sqlx::query_as!(UniversityModel, "select * from universities where id = $1 limit 1", id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("university not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Find an university data by exists column."]
pub async fn find_university_data_by_exists_column(pool: Pool<Postgres>, name: String, major: String) -> anyhow::Result<UniversityModel, ErrorEnum> {
  match sqlx::query_as!(UniversityModel, "select * from universities where name = $1 and major = $2 limit 1", name, major)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("university not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Update an existing university data."]
pub async fn update_university_data(pool: Pool<Postgres>, id: i32, body: UniversitySchema) -> anyhow::Result<UniversityModel, ErrorEnum> {
  match sqlx::query_as!(UniversityModel,
    "update universities set name = $1, major = $2, quantity = $3, description = $4, image = $5, link = $6, alias = $7 where id = $8 returning *",
    body.name, body.major, body.quantity, body.description, body.image, body.link, body.alias, id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("university not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Delete university data by id."]
pub async fn delete_university_data(pool: Pool<Postgres>, id: i32) -> anyhow::Result<UniversityModel, ErrorEnum> {
  match sqlx::query_as!(UniversityModel,
    "delete from universities where id = $1 returning *", id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("university not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}
