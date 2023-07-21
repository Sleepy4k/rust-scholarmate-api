use sqlx::{Pool, Postgres};

use crate::{
  models::auth_model::*,
  schemas::register_schema::*
};

use scholarmate_api::{
  enums::error_enum::ErrorEnum,
  models::auth_model::FilteredUserModel
};

#[doc = "Create an user data."]
pub async fn create_user_data(pool: Pool<Postgres>, body: RegisterSchema) -> anyhow::Result<FilteredUserModel, ErrorEnum> {
  match sqlx::query_as!(FilteredUserModel,
    "insert into users (email, password, role, verified) values ($1, $2, $3, $4) returning id, email, role",
    body.email, body.password, String::from("user"), false)
    .fetch_one(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Find a user data by email."]
pub async fn find_user_data_by_email(pool: Pool<Postgres>, email: String) -> anyhow::Result<UserModel, ErrorEnum> {
  match sqlx::query_as!(UserModel, "select * from users where email = $1 limit 1", email)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("user not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Update an existing user data."]
pub async fn update_user_data(pool: Pool<Postgres>, email: String) -> anyhow::Result<bool, ErrorEnum> {
  match sqlx::query!(
    "update users set verified = true where email = $1",
    email)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(_) => Ok(true),
          None => Ok(false)
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}