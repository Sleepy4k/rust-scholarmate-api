use actix_web::{web::{self}, Responder};

use crate::{
  models::scholarship_model::*,
  schemas::scholarship_schema::*,
  structs::main_struct::AppState,
  helpers::{
    response::response_json,
    validation::check_if_empty,
    parse::convert_vec_to_values
  }
};

#[doc = "Get all scholarship"]
pub async fn get_scholarship(state: web::Data<AppState>) -> impl Responder {
  let data = sqlx::query_as!(DetailScholarshipModel,
    "select s.id, s.name, s.description, s.quantity, s.requirement, u.id as univ_id,
      u.name as univ_name, u.alias as univ_alias, u.description as univ_description,
      u.major as univ_major from scholarships s join universities u on s.univ_id = u.id")
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved scholarship".to_string(),
    result
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

  if check_if_empty(name.to_owned()) || check_if_empty(description.to_owned()) || check_if_empty(major.to_owned()) || check_if_empty(requirement.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let scholarship_exists = sqlx::query_scalar::<_, bool>("select exists(select 1 from scholarships where name = $1) as scholarship_exists")
    .bind(name.clone())
    .fetch_one(&state.db)
    .await
    .unwrap_or(false);

  if scholarship_exists {
    return response_json(
      "failed".to_string(),
      "Scholarship already exists".to_string(),
      vec![]
    )
  }

  let data = sqlx::query_as!(ScholarshipModel,
    "insert into scholarships (name, description, quantity, requirement, univ_id)
      values ($1, $2, $3, $4, $5) returning *",
    name, description, quantity, requirement, univ_id)
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully added scholarship".to_string(),
    result
  )
}

#[doc = "Find scholarship by id"]
pub async fn find_scholarship(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();

  match sqlx::query_as!(ScholarshipModel, "select * from scholarships where id = $1", id)
    .fetch_optional(&state.db)
    .await {
      Ok(Some(scholarship_data)) => {
        let result = convert_vec_to_values(vec![scholarship_data]);

        return response_json(
          "success".to_string(),
          "Successfully retrieved scholarship".to_string(),
          result
        )
      },
      Ok(None) => {
        return response_json(
          "failed".to_string(),
          "Scholarship not found".to_string(),
          vec![]
        )
      }
      Err(_) => return response_json(
        "error".to_string(),
        "Something went wrong".to_string(),
        vec![]
      )
    };
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

  if check_if_empty(name.to_owned()) || check_if_empty(description.to_owned()) || check_if_empty(major.to_owned()) || check_if_empty(requirement.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let scholarship_exists = sqlx::query_scalar::<_, bool>("select exists(select 1 from scholarships where id = $1) as scholarship_exists")
    .bind(id.clone())
    .fetch_one(&state.db)
    .await
    .unwrap_or(false);

  if !scholarship_exists {
    return response_json(
      "failed".to_string(),
      "Scholarship not found".to_string(),
      vec![]
    )
  }

  let data = sqlx::query_as!(ScholarshipModel,
    "update scholarships set name = $1, description = $2, quantity = $3, requirement = $4, univ_id = $5 where id = $6 returning *",
    name, description, quantity, requirement, univ_id, id)
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully updated scholarship".to_string(),
    result
  )
}

#[doc = "Delete scholarship by id"]
pub async fn delete_scholarship(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let scholarship_exists = sqlx::query_scalar::<_, bool>("select exists(select 1 from scholarships where id = $1) as scholarship_exists")
    .bind(id.clone())
    .fetch_one(&state.db)
    .await
    .unwrap_or(false);

  if !scholarship_exists {
    return response_json(
      "failed".to_string(),
      "Scholarship not found".to_string(),
      vec![]
    )
  }

  let data = sqlx::query_as!(ScholarshipModel, "delete from scholarships where id = $1 returning *", id)
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully deleted scholarship".to_string(),
    result
  )
}