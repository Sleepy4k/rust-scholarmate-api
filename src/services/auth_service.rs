use sqlx::{Pool, Postgres};

use crate::{
  repositories::auth_repository::*,
  helpers::parse::convert_vec_to_values,
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  },
};

#[doc = "Display a listing of the resource."]
pub async fn user_get_service(db: Pool<Postgres>) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = get_user_data(db).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_vec_to_values(data.unwrap());
  let body = ResponseDataEnum::ArrayValue(converted_data);

  Ok(body)
}