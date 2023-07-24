use chrono::Utc;
use sqlx::{Pool, Postgres};

use crate::{
  models::room_model::*,
  repositories::room_repository::*
};

use scholarmate_api::{
  repositories::main_repository::check_data,
  helpers::parse::{
    convert_vec_to_values,
    convert_struct_to_value
  },
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

#[doc = "Store a newly created resource in storage."]
pub async fn room_store_service(db: Pool<Postgres>, name: String, members: String) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let current_date_time = Utc::now().naive_utc().date();
  
  let data = create_room_data(db, name, members, current_date_time).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}

#[doc = "Display the specified resource by user id."]
pub async fn room_show_by_user_id_service(db: Pool<Postgres>, id: i32) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = get_room_data(db).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let filtered_data = data.unwrap()
    .into_iter()
    .filter(|item| item.members.contains(&id.to_string()))
    .collect::<Vec<RoomModel>>();

  let converted_data = convert_vec_to_values(filtered_data);
  let body = ResponseDataEnum::ArrayValue(converted_data);

  Ok(body)
}

#[doc = "Remove the specified resource from storage."]
pub async fn room_destroy_service(db: Pool<Postgres>, id: i32) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_room_exist = format!("select 1 from room where id = '{}'", id);
  let is_room_exist = check_data(db.clone(), query_is_room_exist.as_str()).await;

  if is_room_exist.is_err() {
    return Err(is_room_exist.err().unwrap());
  } else if !is_room_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("room not exist")));
  }
  
  let data = delete_room_data(db, id).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}