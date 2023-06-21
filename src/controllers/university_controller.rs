use actix_web::{web::{self}, Responder};

use crate::{
  schemas::university_schema::*,
  structs::main_struct::AppState,
  repositories::{
    university_repository::*,
    main_repository::check_data
  },
  helpers::{
    response::response_json,
    validation::check_if_empty,
    parse::convert_vec_to_values
  }
};

#[doc = "Get all university"]
pub async fn get_university(state: web::Data<AppState>) -> impl Responder {
  let data = fetch_university_data(state.db.to_owned()).await;

  response_json(
    "success".to_string(),
    "Successfully retrieved university".to_string(),
    data
  )
}

#[doc = "Add new university"]
pub async fn add_university(state: web::Data<AppState>, body: web::Json<UniversitySchema>) -> impl Responder {
  let name = body.name.to_owned();
  let major = body.major.to_owned();
  let quantity = body.quantity.to_owned();
  let description = body.description.to_owned();

  if check_if_empty(name.to_owned())
  || check_if_empty(major.to_owned())
  || check_if_empty(quantity.to_owned().to_string())
  || check_if_empty(description.to_owned())
  {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let query_str = format!("select 1 from universities where name = {} and major = {}", name, major);
  let univ_exists = check_data(state.db.clone(), query_str.as_str()).await;

  if univ_exists {
    return response_json(
      "failed".to_string(),
      "University already exist".to_string(),
      vec![]
    )
  }

  let data = insert_university_data(state.db.clone(), body.into_inner()).await;

  response_json(
    "success".to_string(),
    "Successfully added university".to_string(),
    data
  )
}

#[doc = "Find university by id"]
pub async fn find_university(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();

  match fetch_university_data_by_id(state.db.to_owned(), id).await {
    Some(univ_data) => {
      let convert_to_vec = vec![univ_data];
      let data = convert_vec_to_values(convert_to_vec);

      return response_json(
        "success".to_string(),
        "Successfully retrieved university".to_string(),
        data
      )
    },
    None => return response_json(
      "failed".to_string(),
      "University not found".to_string(),
      vec![]
    )
  }
}

#[doc = "Update university by id"]
pub async fn update_university(state: web::Data<AppState>, body: web::Json<UniversitySchema>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let name = body.name.to_owned();
  let major = body.major.to_owned();
  let quantity = body.quantity.to_owned();
  let description = body.description.to_owned();

  if check_if_empty(name.to_owned())
  || check_if_empty(major.to_owned())
  || check_if_empty(quantity.to_owned().to_string())
  || check_if_empty(description.to_owned())
  {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let query_str = format!("select 1 from universities where id = {}", id);
  let univ_exists = check_data(state.db.clone(), query_str.as_str()).await;

  if !univ_exists {
    return response_json(
      "failed".to_string(),
      "University not found".to_string(),
      vec![]
    )
  }

  match fetch_university_data_by_exists_column(state.db.clone(), name, major).await {
    Some(univ_data) => {
      if univ_data.id != id {
        return response_json(
          "failed".to_string(),
          "University already exists".to_string(),
          vec![]
        )
      } else {
        ()
      }
    },
    None => ()
  };

  let data = update_university_data(state.db.clone(), id, body.into_inner()).await;

  response_json(
    "success".to_string(),
    "Successfully updated university".to_string(),
    data
  )
}

#[doc = "Delete university by id"]
pub async fn delete_university(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let query_str = format!("select 1 from universities where id = {}", id);
  let univ_exists = check_data(state.db.clone(), query_str.as_str()).await;

  if !univ_exists {
    return response_json(
      "failed".to_string(),
      "University not found".to_string(),
      vec![]
    )
  }

  let data = delete_university_data(state.db.clone(), id).await;

  response_json(
    "success".to_string(),
    "Successfully deleted university".to_string(),
    data
  )
}