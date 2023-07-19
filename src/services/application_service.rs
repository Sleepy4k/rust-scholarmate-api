use sqlx::{Pool, Postgres};

use crate::{
  schemas::application_schema::ApplicationSchema,
  helpers::parse::{
    convert_vec_to_values,
    convert_struct_to_value
  },
  repositories::{
    application_repository::*, 
    main_repository::check_data
  },
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  },
};

#[doc = "Display a listing of the resource."]
pub async fn application_get_service(db: Pool<Postgres>) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = get_application_data(db).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_vec_to_values(data.unwrap());
  let body = ResponseDataEnum::ArrayValue(converted_data);

  Ok(body)
}

#[doc = "Store a newly created resource in storage."]
pub async fn application_store_service(db: Pool<Postgres>, body: ApplicationSchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_application_exist = format!("select 1 from applications where univ_id = '{}' and student_id = '{}' and major = '{}'", body.univ_id, body.student_id, body.major);
  let is_application_exist = check_data(db.to_owned(), query_is_application_exist.as_str()).await;

  if is_application_exist.is_err() {
    return Err(is_application_exist.err().unwrap());
  } else if is_application_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("application already exist")));
  }

  let update = update_application_data(db.to_owned(), body.univ_id).await;

  if update.is_err() {
    return Err(update.err().unwrap());
  }

  let data = create_application_data(db, body).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}

#[doc = "Display the specified resource."]
pub async fn application_show_service(db: Pool<Postgres>, id: i32) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = find_application_data(db, id).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}