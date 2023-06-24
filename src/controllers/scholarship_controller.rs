use serde_json::Value;
use validator::Validate;
use actix_web::{web, Responder};

use crate::{
  schemas::scholarship_schema::*,
  structs::main_struct::AppState,
  helpers::{
    response::response_json,
    parse::convert_vec_to_values
  },
  repositories::{
    scholarship_repository::*,
    main_repository::check_data
  }
};

#[doc = "Get all scholarship"]
pub async fn get_scholarship(state: web::Data<AppState>) -> impl Responder {
  let data = fetch_scholarship_data(state.db.to_owned()).await;

  response_json(
    "success".to_string(),
    "Successfully retrieved scholarship".to_string(),
    data
  )
}

#[doc = "Add new scholarship"]
pub async fn add_scholarship(state: web::Data<AppState>, body: web::Json<ScholarshipSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![data]
    )
  }

  let query_str = format!("select 1 from scholarships where name = '{}'", body.name);
  let scholarship_exists = check_data(state.db.clone(), query_str.as_str()).await;

  if scholarship_exists {
    return response_json(
      "failed".to_string(),
      "Scholarship already exists".to_string(),
      vec![]
    )
  }

  let data = insert_scholarship_data(state.db.to_owned(), body.into_inner()).await;

  response_json(
    "success".to_string(),
    "Successfully added scholarship".to_string(),
    data
  )
}

#[doc = "Find scholarship by id"]
pub async fn find_scholarship(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();

  match fetch_scholarship_data_by_id(state.db.to_owned(), id).await {
    Some(scholarship_data) => {
      let convert_to_vec = vec![scholarship_data];
      let data = convert_vec_to_values(convert_to_vec);

      return response_json(
        "success".to_string(),
        "Successfully retrieved scholarship".to_string(),
        data
      )
    },
    None => {
      return response_json(
        "failed".to_string(),
        "Scholarship not found".to_string(),
        vec![]
      )
    }
  }
}

#[doc = "Update scholarship by id"]
pub async fn update_scholarship(state: web::Data<AppState>, body: web::Json<ScholarshipSchema>, path: web::Path<i32>) -> impl Responder {
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

  let query_str = format!("select 1 from scholarships where id = '{}'", id);
  let scholarship_exists = check_data(state.db.clone(), query_str.as_str()).await;

  if !scholarship_exists {
    return response_json(
      "failed".to_string(),
      "Scholarship not found".to_string(),
      vec![]
    )
  }

  let data = update_scholarship_data(state.db.to_owned(), id, body.into_inner()).await;

  response_json(
    "success".to_string(),
    "Successfully updated scholarship".to_string(),
    data
  )
}

#[doc = "Delete scholarship by id"]
pub async fn delete_scholarship(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let query_str = format!("select 1 from scholarships where id = '{}'", id);
  let scholarship_exists = check_data(state.db.clone(), query_str.as_str()).await;

  if !scholarship_exists {
    return response_json(
      "failed".to_string(),
      "Scholarship not found".to_string(),
      vec![]
    )
  }

  let data = delete_scholarship_data(state.db.to_owned(), id).await;

  response_json(
    "success".to_string(),
    "Successfully deleted scholarship".to_string(),
    data
  )
}
