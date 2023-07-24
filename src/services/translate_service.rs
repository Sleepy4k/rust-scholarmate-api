use sqlx::{Pool, Postgres};

use crate::{
  helpers::parse::convert_vec_to_values,
  repositories::translate_repository::*,
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

#[doc = "Display a listing of the resource."]
pub async fn translate_index_service(db: Pool<Postgres>) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = get_translate_data(db).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_vec_to_values(data.unwrap());
  let body = ResponseDataEnum::ArrayValue(converted_data);

  Ok(body)
}

#[doc = "Display the specified resource by exist column."]
pub async fn translate_show_by_table_name_service(db: Pool<Postgres>, table: String) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = find_translate_data_by_table_name(db, table).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_vec_to_values(data.unwrap());
  let body = ResponseDataEnum::ArrayValue(converted_data);

  Ok(body)
}
