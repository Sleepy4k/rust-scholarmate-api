use actix_web::{web::{self}, Responder};

use crate::{
  schemas::application_schema::*,
  structs::main_struct::AppState,
  repositories::{
    application_repository::*,
    main_repository::check_data,
    university_repository::fetch_university_data_by_id,
    scholarship_repository::fetch_scholarship_data_by_id
  },
  helpers::{
    response::response_json,
    validation::check_if_empty
  }
};

#[doc = "Get all application"]
pub async fn get_application(state: web::Data<AppState>) -> impl Responder {
  let data = fetch_application_data(state.db.to_owned()).await;

  response_json(
    "success".to_string(),
    "Successfully retrieved application".to_string(),
    data
  )
}

#[doc = "Get user application"]
pub async fn get_my_application(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let data = fetch_application_data_by_id(state.db.to_owned(), id).await;

  response_json(
    "success".to_string(),
    "Successfully retrieved application".to_string(),
    data
  )
}

#[doc = "Add application"]
pub async fn add_application(state: web::Data<AppState>, body: web::Json<ApplicationSchema>) -> impl Responder {
  let major = body.major.to_owned();
  let status = body.status.to_owned();
  let univ_id = body.univ_id.to_owned();
  let student_id = body.student_id.to_owned();
  let scholarship_id = body.scholarship_id.to_owned();

  if check_if_empty(major.to_owned())
    || check_if_empty(status.to_owned())
    || check_if_empty(univ_id.to_owned().to_string())
    || check_if_empty(student_id.to_owned().to_string())
    || check_if_empty(scholarship_id.to_owned().to_string())
  {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  match fetch_university_data_by_id(state.db.clone(), univ_id).await {
    Some(univ_data) => {
      if univ_data.major != major {
        return response_json(
          "failed".to_string(),
          "Major not found".to_string(),
          vec![]
        );
      } else if univ_data.quantity == 0 {
        return response_json(
          "failed".to_string(),
          "University quota is full".to_string(),
          vec![]
        )
      } else {
        ()
      }
    }
    None => return response_json(
      "failed".to_string(),
      "University not found".to_string(),
      vec![]
    )
  };

  match fetch_scholarship_data_by_id(state.db.clone(), scholarship_id).await {
    Some(scholarship_data) => {
      if scholarship_data.quantity == 0 {
        return response_json(
          "failed".to_string(),
          "Scholarship quota is full".to_string(),
          vec![]
        )
      } else {
        ()
      }
    }
    None => return response_json(
      "failed".to_string(),
      "Scholarship not found".to_string(),
      vec![]
    )
  };

  let query_str = format!("select 1 from applications where univ_id = {} and student_id = {} and major = {}", univ_id, student_id, major);
  let app_exist = check_data(state.db.clone(), query_str.as_str()).await;

  if app_exist {
    return response_json(
      "failed".to_string(),
      "Application already exist".to_string(),
      vec![]
    )
  }
  
  update_application_data(state.db.clone(), univ_id).await;

  let app_data: ApplicationSchema = body.into_inner();
  let data = insert_application_data(state.db.to_owned(), app_data).await;

  response_json(
    "success".to_string(),
    "Successfully added application".to_string(),
    data
  )
}
