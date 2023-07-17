use sqlx::{Pool, Postgres};

use crate::{
  repositories::forum_repository::*,
  helpers::parse::convert_vec_to_values,
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  },
};

#[doc = "Display the specified resource."]
pub async fn forum_show_service(db: Pool<Postgres>, id: i32) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = find_forum_data(db, id).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_vec_to_values(data.unwrap());
  let body = ResponseDataEnum::ArrayValue(converted_data);

  Ok(body)
}