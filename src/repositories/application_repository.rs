use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::{
  enums::error_enum::ErrorEnum,
  schemas::application_schema::ApplicationSchema,
  models::application_model::{
    ApplicationModel,
    DetailApplicationModel
  }
};

#[doc = "Get all application data."]
pub async fn get_application_data(pool: Pool<Postgres>) -> anyhow::Result<Vec<ApplicationModel>, ErrorEnum> {
  match sqlx::query_as!(ApplicationModel, "select * from applications")
    .fetch_all(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Create an application data."]
pub async fn create_application_data(pool: Pool<Postgres>, body: ApplicationSchema) -> anyhow::Result<ApplicationModel, ErrorEnum> {
  match sqlx::query_as!(ApplicationModel, "insert into applications (student_id, univ_id, status) values ($1, $2, $3) returning *",
    body.student_id, body.univ_id, body.status)
    .fetch_one(&pool)
    .await {
      Ok(data) => Ok(data),
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}

#[doc = "Find an application data by id."]
pub async fn find_application_data(pool: Pool<Postgres>, id: i32) -> anyhow::Result<DetailApplicationModel, ErrorEnum> {
  match sqlx::query_as!(DetailApplicationModel, "select app.id, univ.id as univ_id, univ.name, univ.major, univ.image, app.status from applications app
      join universities univ on app.univ_id = univ.id where student_id = $1
      order by univ.id desc"
    , id)
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


#[doc = "Update an existing application data."]
pub async fn update_application_data(pool: Pool<Postgres>, id: i32) -> anyhow::Result<Vec<Value>, ErrorEnum> {
  match sqlx::query!(
    "with updated_university as (update universities set quantity = quantity - 1 where id = $1 returning id)
    update scholarships set quantity = quantity - 1 where univ_id in (select id from updated_university)",
    id)
    .fetch_optional(&pool)
    .await {
      Ok(optional_data) => {
        match optional_data {
          Some(_) => Ok(vec![]),
          None => Err(ErrorEnum::CustomError(String::from("scholarship not exist")))
        }
      },
      Err(_) => Err(ErrorEnum::InternalServerError)
    }
}