use sqlx::{Pool, Postgres};

use crate::{
  enums::error_enum::ErrorEnum,
  models::auth_model::FilteredUserModel
};

#[doc = "Get all user data."]
pub async fn get_user_data(pool: Pool<Postgres>) -> anyhow::Result<Vec<FilteredUserModel>, ErrorEnum> {
  match sqlx::query_as!(FilteredUserModel, "select id, email, role from users")
    .fetch_all(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}
