use sqlx::{Pool, Postgres};

use crate::{
  models::otp_model::*,
  schemas::otp_schema::*,
};

use scholarmate_api::enums::error_enum::ErrorEnum;

#[doc = "Create a token data."]
pub async fn create_token_data(pool: Pool<Postgres>, body: OTPVerificationSchema) -> anyhow::Result<OTPModel, ErrorEnum> {
  match sqlx::query_as!(OTPModel,
    "insert into tokens (token, otp, email) values ($1, $2, $3) returning *",
    body.token, body.otp, body.email)
    .fetch_one(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Delete token data by token"]
pub async fn delete_token_data(pool: Pool<Postgres>, token: String, email: String) -> anyhow::Result<OTPModel, ErrorEnum> {
  match sqlx::query_as!(OTPModel,
    "delete from tokens where token = $1 or email = $2 returning *", token, email)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(data),
          None => Err(ErrorEnum::CustomError(String::from("token not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}