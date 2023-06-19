use actix_web::{web::{self}, Responder};
use std::{env, time::{SystemTime, UNIX_EPOCH}};
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::{helpers::{response::{response_json, response_json_with_cookie}, parse::convert_vec_to_values, validation::check_if_empty, auth::*}, structs::{auth_struct::*, student_struct::StudentStruct, main_struct::*}};

#[doc = "Verify user credentials and return token"]
pub async fn login(state: web::Data<AppState>, body: web::Json<LoginStruct>) -> impl Responder {
  let email = body.email.to_owned();
  let password = body.password.to_owned();

  if check_if_empty(email.to_owned()) || check_if_empty(password.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let user = match sqlx::query!("select * from users where email = $1 limit 1", email)
    .fetch_optional(&state.db)
    .await {
      Ok(Some(user)) => user,
      Ok(None) => {
        return response_json(
          "failed".to_string(),
          "Account not found".to_string(),
          vec![]
        );
      }
      Err(_) => return response_json(
        "error".to_string(),
        "Something went wrong".to_string(),
        vec![]
      )
    };

  if !verify_password(password.as_str(), &user.password) {
    return response_json(
      "failed".to_string(),
      "Email or password is wrong".to_string(),
      vec![]
    )
  }

  let token_time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

  let token_value = TokenStruct {
    id: user.id,
    email: user.email.clone(),
    role: user.role.clone(),
    iat: token_time,
    exp: token_time.saturating_add(604800),
  };

  let jwt_title = env::var("JWT_TOKEN_TITLE").unwrap_or_else(|_| String::from("auth_jwt_secret"));
  let jwt_secret = env::var("JWT_TOKEN_SECRET").unwrap_or_else(|_| String::from("secret"));
  let key = EncodingKey::from_secret(jwt_secret.as_ref());
  let token = encode(&Header::default(), &token_value, &key).unwrap_or_else(|_| String::new());

  let student = sqlx::query_as!(StudentStruct, "select * from students where email = $1", user.email.clone())
    .fetch_optional(&state.db)
    .await
    .unwrap();

  let detail_user = convert_vec_to_values(vec![
    DetailUserStruct {
      id: user.id,
      email: user.email,
      role: user.role.clone(),
      student: student,
    }
  ]);

  response_json_with_cookie(
    "success".to_string(),
    "Successfully logged in".to_string(),
    detail_user,
    "set".to_string(),
    jwt_title,
    token,
  )
}

#[doc = "Register new user"]
pub async fn register(state: web::Data<AppState>, body: web::Json<RegisterStruct>) -> impl Responder {
  let email = body.email.to_owned();
  let password = body.password.to_owned();
  let role = body.role.to_owned();

  if check_if_empty(email.to_owned()) || check_if_empty(password.to_owned()) || check_if_empty(role.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  match sqlx::query!("select id from users where email = $1", email.to_owned())
    .fetch_optional(&state.db)
    .await {
      Ok(Some(_)) => {
        return response_json(
          "failed".to_string(),
          "Email already exists".to_string(),
          vec![]
        )
      },
      Ok(None) => (),
      Err(_) => return response_json(
        "error".to_string(),
        "Something went wrong".to_string(),
        vec![]
      )
    };

  let hashed_password = hash_password(password.as_str());

  match sqlx::query_as!(UserStruct,
    "insert into users (email, password, role) values ($1, $2, $3) returning id, email, role",
    email.to_owned(), hashed_password, role.to_owned()
  ).fetch_one(&state.db).await {
    Ok(data) => {
      let detail_user = convert_vec_to_values(vec![data]);

      response_json(
        "success".to_string(),
        "Successfully registered".to_string(),
        detail_user
      )
    },
    Err(_) => response_json(
      "error".to_string(),
      "Something went wrong".to_string(),
      vec![]
    )
  }
}

#[doc = "Logout user"]
pub async fn logout() -> impl Responder {
  let jwt_title = env::var("JWT_TOKEN_TITLE").unwrap_or_else(|_| String::from("auth_jwt_secret"));

  response_json_with_cookie(
    "success".to_string(),
    "Successfully logged out".to_string(),
    vec![],
    "remove".to_string(),
    jwt_title,
    "".to_string()
  )
}
