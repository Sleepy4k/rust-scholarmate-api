use serde_json::Value;
use actix_web::{web::{self}, Responder};

use crate::{helpers::{response::*, database::connect_postgres, parse::*}, structs::application_struct::*};

#[doc = "Get all application"]
pub async fn get_application() -> impl Responder {
  let pool = connect_postgres().await;
  let data = sqlx::query_as!(ApplicationStruct, "select * from applications")
    .fetch_all(&pool)
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
pub async fn get_my_application(arg: web::Path<i32>) -> impl Responder {
  let id = arg.to_owned();

  let pool = connect_postgres().await;

  let data = sqlx::query_as!(DetailApplicationStruct,
    "select applications.id, universities.id as univ_id, universities.name, universities.major, universities.image, applications.status from applications
      join universities on applications.univ_id = universities.id where student_id = $1
      order by universities.id desc"
    , id)
    .fetch_all(&pool)
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
pub async fn add_application(data: web::Json<Value>) -> impl Responder {
  let schoolarship_id = to_i32(map_get("schoolarship_id", data.to_owned()));
  let univ_id = to_i32(map_get("univ_id", data.to_owned()));
  let student_id = to_i32(map_get("student_id", data.to_owned()));
  let status = to_str(map_get("status", data.to_owned()));
  let major = to_str(map_get("major", data.to_owned()));

  let pool = connect_postgres().await;

  match sqlx::query!("select major, quantity from universities where id = $1", univ_id)
    .fetch_optional(&pool)
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

  match sqlx::query!("select major, quantity from schoolarships where id = $1", schoolarship_id)
    .fetch_optional(&pool)
    .await {
      Ok(Some(schoolarship_data)) => {
        if schoolarship_data.major != major {
          return response_json(
            "failed".to_string(),
            "Major not found".to_string(),
            vec![]
          );
        } else if schoolarship_data.quantity == 0 {
          return response_json(
            "failed".to_string(),
            "Schoolarship quota is full".to_string(),
            vec![]
          )
        } else {
          ()
        }
      }
      Ok(None) => return response_json(
        "failed".to_string(),
        "Schoolarship not found".to_string(),
        vec![]
      ),
      Err(_) => return response_json(
        "error".to_string(),
        "Something went wrong".to_string(),
        vec![]
      )
    };

  match sqlx::query!("select * from applications where univ_id = $1 and student_id = $2 and major = $3", univ_id, student_id, major)
    .fetch_optional(&pool)
    .await {
      Ok(Some(_)) => return response_json(
        "failed".to_string(),
        "Application already exists".to_string(),
        vec![]
      ),
      Ok(None) => (),
      Err(_) => return response_json(
        "error".to_string(),
        "Something went wrong".to_string(),
        vec![]
      )
    };

  let data = sqlx::query_as!(ApplicationStruct,
    "insert into applications (univ_id, student_id, status, major) values ($1, $2, $3, $4) returning *"
    , univ_id, student_id, status, major)
    .fetch_all(&pool)
    .await
    .unwrap();

  sqlx::query!("update universities set quantity = quantity - 1 where id = $1", univ_id)
    .execute(&pool)
    .await
    .unwrap();

  sqlx::query!("update schoolarships set quantity = quantity - 1 where univ_id = $1", univ_id)
    .execute(&pool)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully added application".to_string(),
    result
  )
}
