use serde_json::Value;
use validator::Validate;
use actix_web::{web, Responder};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
  schemas::login_schema::*,
  models::auth_model::DetailUserModel,
  repositories::auth_repository::fetch_user_data_by_email,
};

use scholarmate_api::{
  repositories::student_repository::fetch_student_data_by_exists_column,
  structs::{
    main_struct::AppState,
    auth_struct::TokenStruct
  },
  helpers::{
    hashing::verify_password,
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

  if user.verified == false {
    return response_json(
      String::from("failed"),
      String::from("please verify your account first"),
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