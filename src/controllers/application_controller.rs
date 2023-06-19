use actix_web::{web::{self}, Responder};

use crate::{
  models::application_model::*,
  schemas::application_schema::*,
  structs::main_struct::AppState,
  helpers::{response::response_json, parse::convert_vec_to_values}
};

#[doc = "Get all application"]
pub async fn get_application(state: web::Data<AppState>) -> impl Responder {
  let data = sqlx::query_as!(ApplicationModel, "select * from applications")
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved application".to_string(),
    result
  )
}

#[doc = "Get user application"]
pub async fn get_my_application(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let data = sqlx::query_as!(DetailApplicationModel,
    "select app.id, univ.id as univ_id, univ.name, univ.major, univ.image, app.status from applications app
      join universities univ on app.univ_id = univ.id where student_id = $1
      order by univ.id desc"
    , id)
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved application".to_string(),
    result
  )
}

#[doc = "Add application"]
pub async fn add_application(state: web::Data<AppState>, body: web::Json<ApplicationSchema>) -> impl Responder {
  let major = body.major.to_owned();
  let status = body.status.to_owned();
  let univ_id = body.univ_id.to_owned();
  let student_id = body.student_id.to_owned();
  let scholarship_id = body.scholarship_id.to_owned();

  match sqlx::query!("select major, quantity from universities where id = $1", univ_id)
    .fetch_optional(&state.db)
    .await {
      Ok(Some(univ_data)) => {
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
      Ok(None) => return response_json(
        "failed".to_string(),
        "University not found".to_string(),
        vec![]
      ),
      Err(_) => return response_json(
        "error".to_string(),
        "Something went wrong".to_string(),
        vec![]
      )
    };

  match sqlx::query!("select quantity from scholarships where id = $1", scholarship_id)
    .fetch_optional(&state.db)
    .await {
      Ok(Some(scholarship_data)) => {
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
      Ok(None) => return response_json(
        "failed".to_string(),
        "Scholarship not found".to_string(),
        vec![]
      ),
      Err(_) => return response_json(
        "error".to_string(),
        "Something went wrong".to_string(),
        vec![]
      )
    };

  let app_exists = sqlx::query_scalar::<_, bool>("select exists(select 1 from applications where univ_id = $1 and student_id = $2 and major = $3) as app_exists")
    .bind(univ_id.clone())
    .bind(student_id.clone())
    .bind(major.clone())
    .fetch_one(&state.db)
    .await
    .unwrap_or(false);

  if app_exists {
    return response_json(
      "failed".to_string(),
      "Application already exists".to_string(),
      vec![]
    )
  }
  
  sqlx::query!("with updated_university as (update universities set quantity = quantity - 1 where id = $1 returning id)
    update scholarships set quantity = quantity - 1 where univ_id in (select id from updated_university)", univ_id)
    .execute(&state.db)
    .await
    .unwrap();

  let data = sqlx::query_as!(ApplicationModel,
    "insert into applications (univ_id, student_id, status, major) values ($1, $2, $3, $4) returning *"
    , univ_id, student_id, status, major)
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully added application".to_string(),
    result
  )
}
