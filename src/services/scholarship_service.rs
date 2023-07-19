use sqlx::{Pool, Postgres};

use crate::{
  schemas::scholarship_schema::*,
  helpers::parse::{
    convert_vec_to_values,
    convert_struct_to_value
  },
  repositories::{
    scholarship_repository::*,
    main_repository::check_data
  },
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

#[doc = "Display a listing of the resource."]
pub async fn scholarship_index_service(db: Pool<Postgres>) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = get_scholarship_data(db).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_vec_to_values(data.unwrap());
  let body = ResponseDataEnum::ArrayValue(converted_data);

  Ok(body)
}

#[doc = "Store a newly created resource in storage."]
pub async fn scholarship_store_service(db: Pool<Postgres>, body: ScholarshipSchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_scholarship_exist = format!("select 1 from scholarships where name = '{}'", body.name);
  let is_scholarship_exist = check_data(db.clone(), query_is_scholarship_exist.as_str()).await;

  if is_scholarship_exist.is_err() {
    return Err(is_scholarship_exist.err().unwrap());
  } else if is_scholarship_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("scholarship already exist")));
  }
  
  let data = create_scholarship_data(db, body).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}

#[doc = "Display the specified resource."]
pub async fn scholarship_show_service(db: Pool<Postgres>, id: i32) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = find_scholarship_data(db, id).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}

#[doc = "Update the specified resource in storage."]
pub async fn scholarship_update_service(db: Pool<Postgres>, id: i32, body: ScholarshipSchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_scholarship_exist = format!("select 1 from scholarships where id = '{}'", id);
  let is_scholarship_exist = check_data(db.clone(), query_is_scholarship_exist.as_str()).await;

  if is_scholarship_exist.is_err() {
    return Err(is_scholarship_exist.err().unwrap());
  } else if !is_scholarship_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("scholarship not exist")));
  }
  
  let data = update_scholarship_data(db, id, body).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}

#[doc = "Remove the specified resource from storage."]
pub async fn scholarship_destroy_service(db: Pool<Postgres>, id: i32) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_scholarship_exist = format!("select 1 from scholarships where id = '{}'", id);
  let is_scholarship_exist = check_data(db.clone(), query_is_scholarship_exist.as_str()).await;

  if is_scholarship_exist.is_err() {
    return Err(is_scholarship_exist.err().unwrap());
  } else if !is_scholarship_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("scholarship not exist")));
  }
  
  let data = delete_scholarship_data(db, id).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}