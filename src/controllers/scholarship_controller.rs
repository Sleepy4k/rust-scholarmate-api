use actix_web::{web::{self}, Responder};

use crate::{
  schemas::scholarship_schema::*,
  structs::main_struct::AppState,
  repositories::{
    scholarship_repository::*,
    main_repository::check_data
  },
  helpers::{
    response::response_json,
    validation::check_if_empty,
    parse::convert_vec_to_values
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
  let name = body.name.to_owned();
  let major = body.major.to_owned();
  let univ_id = body.univ_id.to_owned();
  let quantity = body.quantity.to_owned();
  let description = body.description.to_owned();
  let requirement = body.requirement.to_owned();

  if check_if_empty(name.to_owned())
    || check_if_empty(major.to_owned())
    || check_if_empty(univ_id.to_owned().to_string())
    || check_if_empty(quantity.to_owned().to_string())
    || check_if_empty(description.to_owned())
    || check_if_empty(requirement.to_owned())
  {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let query_str = format!("select 1 from scholarships where name = {}", name);
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
  let name = body.name.to_owned();
  let major = body.major.to_owned();
  let univ_id = body.univ_id.to_owned();
  let quantity = body.quantity.to_owned();
  let description = body.description.to_owned();
  let requirement = body.requirement.to_owned();

  if check_if_empty(name.to_owned())
    || check_if_empty(major.to_owned())
    || check_if_empty(univ_id.to_owned().to_string())
    || check_if_empty(quantity.to_owned().to_string())
    || check_if_empty(description.to_owned())
    || check_if_empty(requirement.to_owned())
  {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let query_str = format!("select 1 from scholarships where id = {}", id);
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
  let query_str = format!("select 1 from scholarships where id = {}", id);
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
