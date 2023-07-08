use serde_json::Value;
use validator::Validate;
use actix_web::{web, Responder};

use crate::{
  schemas::register_schema::RegisterSchema,
  repositories::auth_repository::{
    insert_user_data,
    fetch_user_data_by_email
  },
};

use scholarmate_api::{
  structs::main_struct::AppState,
  helpers::hashing::hash_password,
  helpers::response::response_json
};

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