use sqlx::{Pool, Postgres};

use crate::{
  schemas::student_schema::*,
  helpers::parse::{
    convert_vec_to_values,
    convert_struct_to_value
  },
  repositories::{
    student_repository::*,
    main_repository::check_data
  },
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

#[doc = "Display a listing of the resource."]
pub async fn student_index_service(db: Pool<Postgres>) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = get_student_data(db).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_vec_to_values(data.unwrap());
  let body = ResponseDataEnum::ArrayValue(converted_data);

  Ok(body)
}

#[doc = "Store a newly created resource in storage."]
pub async fn student_store_service(db: Pool<Postgres>, body: StudentSchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_student_exist = format!("select 1 from students where email = '{}' or phone = '{}' or register_number = '{}'", body.email, body.phone, body.register_number);
  let is_student_exist = check_data(db.clone(), query_is_student_exist.as_str()).await;

  if is_student_exist.is_err() {
    return Err(is_student_exist.err().unwrap());
  } else if is_student_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("student already exist")));
  }
  
  let data = create_student_data(db, body).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}

#[doc = "Display the specified resource."]
pub async fn student_show_service(db: Pool<Postgres>, id: i32) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = find_student_data(db, id).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}

#[doc = "Display the specified resource by exist column."]
pub async fn student_show_by_exist_column_service(db: Pool<Postgres>, email: String, phone: String, register_number: String) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let data = find_student_data_by_exists_column(db, email, phone, register_number).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}

#[doc = "Update the specified resource in storage."]
pub async fn student_update_service(db: Pool<Postgres>, id: i32, body: StudentSchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_student_exist = format!("select 1 from students where id = '{}'", id);
  let is_student_exist = check_data(db.clone(), query_is_student_exist.as_str()).await;

  if is_student_exist.is_err() {
    return Err(is_student_exist.err().unwrap());
  } else if !is_student_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("student not exist")));
  }

  match find_student_data_by_exists_column(db.clone(), body.email.to_owned(), body.phone.to_owned(), body.register_number.to_owned()).await {
    Ok(data) => {
      if data.id != id {
        return Err(ErrorEnum::CustomError(String::from("student already exist")));
      }
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(_) => (),
        _ => return Err(ErrorEnum::InternalServerError)
      }
    }
  }

  let data = update_student_data(db, id, body).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}

#[doc = "Remove the specified resource from storage."]
pub async fn student_destroy_service(db: Pool<Postgres>, id: i32) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  let query_is_student_exist = format!("select 1 from students where id = '{}'", id);
  let is_student_exist = check_data(db.clone(), query_is_student_exist.as_str()).await;

  if is_student_exist.is_err() {
    return Err(is_student_exist.err().unwrap());
  } else if !is_student_exist.unwrap() {
    return Err(ErrorEnum::CustomError(String::from("student not exist")));
  }
  
  let data = delete_student_data(db, id).await;

  if data.is_err() {
    return Err(data.err().unwrap());
  }

  let converted_data = convert_struct_to_value(data.unwrap());
  let body = ResponseDataEnum::SingleValue(converted_data);

  Ok(body)
}