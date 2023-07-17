use sqlx::{Pool, Postgres};

use crate::{
  enums::error_enum::ErrorEnum,
  models::forum_model::ForumModel
};

#[doc = "Find forum data by id."]
pub async fn find_forum_data(pool: Pool<Postgres>, id: i32) -> anyhow::Result<Vec<ForumModel>, ErrorEnum> {
  match sqlx::query_as!(ForumModel,
    "select universities.alias, forums.message, universities.image, universities.link from forums
      join universities on forums.univ_id = universities.id where student_id = $1
      order by forums.id desc"
    , id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(data) => Ok(vec![data]),
          None => Ok(vec![])
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}
