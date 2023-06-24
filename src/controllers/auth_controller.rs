use serde_json::Value;
use validator::Validate;
use actix_web::{web, Responder};
use std::{env, time::{SystemTime, UNIX_EPOCH}};
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::{
  models::auth_model::*,
  schemas::auth_schema::*,
  structs::{
    main_struct::AppState,
    auth_struct::TokenStruct
  },
  repositories::{
    auth_repository::*,
    student_repository::fetch_student_data_by_exists_column
  },
  helpers::{
    auth::*,
    parse::convert_vec_to_values,
    response::{response_json, response_json_with_cookie}
  }
};

#[doc = "Verify user credentials and return token"]
pub async fn login(state: web::Data<AppState>, body: web::Json<LoginSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![data]
    )
  }

  let user = match fetch_user_data_by_email(state.db.clone(), body.email.to_owned()).await {
    Some(user) => user,
    None => return response_json(
      "failed".to_string(),
      "Account not found".to_string(),
      vec![]
    )
  };

  if !verify_password(body.password.as_str(), &user.password) {
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

  let student = fetch_student_data_by_exists_column(state.db.clone(), user.email.to_owned(), String::new(), String::new()).await;

  let detail_user = convert_vec_to_values(vec![
    DetailUserModel {
      id: user.id,
      email: user.email,
      role: user.role,
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
pub async fn register(state: web::Data<AppState>, body: web::Json<RegisterSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![data]
    )
  }

  match fetch_user_data_by_email(state.db.clone(), body.email.to_owned()).await {
    Some(_) => return response_json(
      "failed".to_string(),
      "Email already exists".to_string(),
      vec![]
    ),
    None => ()
  };

  let hashed_password = hash_password(body.password.as_str());

  let user_data = RegisterSchema {
    email: body.email.to_owned(),
    password: hashed_password.to_owned(),
    role: body.role.to_owned(),
  };

  let data = insert_user_data(state.db.clone(), user_data).await;

  response_json(
    "success".to_string(),
    "Successfully registered".to_string(),
    data
  )
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
