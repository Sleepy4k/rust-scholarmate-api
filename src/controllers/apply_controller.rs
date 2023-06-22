use serde_json::Value;
use validator::Validate;
use actix_web::{web::{self}, Responder};

use crate::{
  schemas::student_schema::*,
  structs::main_struct::AppState,
  helpers::response::response_json,
  repositories::{
    main_repository::check_data,
    student_repository::{
      insert_student_data,
      update_student_data_by_id,
      fetch_student_data_by_exists_column
    }
  }
};

#[doc = "Add new student"]
pub async fn post_apply(state: web::Data<AppState>, body: web::Json<StudentSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![data]
    )
  }

  let query_str = format!("select 1 from students where email = '{}' or phone = '{}' or register_number = '{}'", body.email, body.phone, body.register_number);
  let student_exist = check_data(state.db.clone(), query_str.as_str()).await;

  if student_exist {
    return response_json(
      "failed".to_string(),
      "Student already exist".to_string(),
      vec![]
    )
  }

  let data = insert_student_data(state.db.clone(), body.into_inner()).await;

  response_json(
    "success".to_string(),
    "Successfully added student".to_string(),
    data
  )
}

#[doc = "Update student data"]
pub async fn put_apply(state: web::Data<AppState>, body: web::Json<StudentSchema>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![data]
    )
  }

  let query_str = format!("select 1 from students where id = '{}'", id);
  let student_exist = check_data(state.db.clone(), query_str.as_str()).await;

  if !student_exist {
    return response_json(
      "failed".to_string(),
      "Student not found".to_string(),
      vec![]
    )
  }
  
  match fetch_student_data_by_exists_column(state.db.clone(), body.email.to_owned(), body.phone.to_owned(), body.register_number.to_owned()).await {
    Some(student_data) => {
      if student_data.id != id {
        return response_json(
          "failed".to_string(),
          "Student already exists".to_string(),
          vec![]
        )
      } else {
        ()
      }
    },
    None => ()
  };

  let data = update_student_data_by_id(state.db.clone(), id, body.into_inner()).await;

  response_json(
    "success".to_string(),
    "Successfully updated student".to_string(),
    data
  )
}