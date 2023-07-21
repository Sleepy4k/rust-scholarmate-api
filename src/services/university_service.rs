use sqlx::{Pool, Postgres};

use crate::{
  schemas::university_schema::*,
  helpers::parse::{
    convert_vec_to_values,
    convert_struct_to_value
  },
  repositories::{
    university_repository::*,
    main_repository::check_data
  },
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

#[doc = "Display a listing of the resource."]
pub async fn university_index_service(db: Pool<Postgres>) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = get_university_data(db).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_vec_to_values(data.unwrap());
  let body = ResponseDataEnum::ArrayValue(converted_data);

  Ok(body)
}

#[doc = "Store a newly created resource in storage."]
pub async fn university_store_service(db: Pool<Postgres>, body: UniversitySchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_univ_exist = format!("select 1 from universities where name = '{}' and major = '{}'", body.name, body.major);
  let is_univ_exist = check_data(db.clone(), query_is_univ_exist.as_str()).await;

  if is_univ_exist.is_err() {
    return Err(is_univ_exist.err().unwrap());
  } else if is_univ_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("university already exist")));
  }
  
  let data = create_university_data(db, body).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}

#[doc = "Display the specified resource."]
pub async fn university_show_service(db: Pool<Postgres>, id: i32) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = find_university_data(db, id).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}

#[doc = "Update the specified resource in storage."]
pub async fn university_update_service(db: Pool<Postgres>, id: i32, body: UniversitySchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_univ_exist = format!("select 1 from universities where id = '{}'", id);
  let is_univ_exist = check_data(db.clone(), query_is_univ_exist.as_str()).await;

  if is_univ_exist.is_err() {
    return Err(is_univ_exist.err().unwrap());
  } else if !is_univ_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("university not exist")));
  }
  
  match find_university_data_by_exists_column(db.clone(), body.name.clone(), body.major.clone()).await {
    Ok(data) => {
      if data.id != id {
        return Err(ErrorEnum::CustomError(String::from("university already exist")));
      } else {
        ()
      }
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(_) => (),
        _ => return Err(ErrorEnum::InternalServerError)
      }
    }
  }

  let data = update_university_data(db, id, body).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}

#[doc = "Remove the specified resource from storage."]
pub async fn university_destroy_service(db: Pool<Postgres>, id: i32) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_univ_exist = format!("select 1 from universities where id = '{}'", id);
  let is_univ_exist = check_data(db.clone(), query_is_univ_exist.as_str()).await;

  if is_univ_exist.is_err() {
    return Err(is_univ_exist.err().unwrap());
  } else if !is_univ_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("university not exist")));
  }
  
  let data = delete_university_data(db, id).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}