use serde_json::Value;
use validator::Validate;
use actix_web::{web, Responder};
use std::time::{SystemTime, UNIX_EPOCH};

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
      String::from("failed"),
      String::from("please fill all fields"),
      vec![data]
    )
  }

  let user = match fetch_user_data_by_email(state.db.clone(), body.email.to_owned()).await {
    Some(user) => user,
    None => return response_json(
      String::from("failed"),
      String::from("account not found"),
      vec![]
    )
  };

  if !verify_password(body.password.as_str(), &user.password) {
    return response_json(
      String::from("failed"),
      String::from("email or password is wrong"),
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

  let token = token_value.to_jwt();
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
    String::from("success"),
    String::from("successfully logged in"),
    detail_user,
    token
  )
}

#[doc = "Register new user"]
pub async fn register(state: web::Data<AppState>, body: web::Json<RegisterSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return response_json(
      String::from("failed"),
      String::from("please fill all fields"),
      vec![data]
    )
  }

  match fetch_user_data_by_email(state.db.clone(), body.email.to_owned()).await {
    Some(_) => return response_json(
      String::from("failed"),
      String::from("email already exists"),
      vec![]
    ),
    None => ()
  };

  if body.password.len() < 8 {
    return response_json(
      String::from("failed"),
      String::from("password must be at least 8 characters"),
      vec![]
    )
  }

  if body.password != body.password_confirmation {
    return response_json(
      String::from("failed"),
      String::from("password and confirm password must be same"),
      vec![]
    )
  }

  let hashed_password = hash_password(body.password.as_str());

  let user_data = RegisterSchema {
    email: body.email.to_owned(),
    role: body.role.to_owned(),
    password: hashed_password,
    password_confirmation: String::new(),
  };

  let data = insert_user_data(state.db.clone(), user_data).await;

  response_json(
    String::from("success"),
    String::from("successfully registered"),
    data
  )
}

#[doc = "Logout user"]
pub async fn logout() -> impl Responder {
  response_json_with_cookie(
    String::from("success"),
    String::from("successfully logged out"),
    vec![],
    String::new()
  )
}
