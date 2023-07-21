use sqlx::{Pool, Postgres};

use crate::{
  repositories::auth_repository::*,
  helpers::parse::generate_secret_key,
  schemas::register_schema::RegisterSchema
};

use scholarmate_api::{
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  },
  helpers::{
    hashing::hash_password,
    parse::{
      get_email_parts,
      convert_struct_to_value
    }
  }
};

#[doc = "Display a listing of the resource."]
pub async fn register_index_service(db: Pool<Postgres>, body: RegisterSchema) -> anyhow::Result<ResponseDataEnum, ErrorEnum> {
  match find_user_data_by_email(db.clone(), body.email.to_owned()).await {
    Ok(_) => {
      return Err(ErrorEnum::CustomError(String::from("email already exists")));
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(_) => (),
        _ => return Err(ErrorEnum::InternalServerError)
      }
    }
  };

  if body.password.len() < 8 || body.password_confirmation.len() < 8 {
    return Err(ErrorEnum::CustomError(String::from("password and password confirmation must be at least 8 characters")));
  }

  if body.password != body.password_confirmation {
    return Err(ErrorEnum::CustomError(String::from("password and password confirmation must be same")));
  }

  let user_name = get_email_parts(body.email.as_str())[0];
  let password_salt = generate_secret_key(user_name);
  let password_hashed = hash_password(body.password.as_str(), password_salt.as_bytes());

  let new_user = RegisterSchema {
    email: body.email,
    password: password_hashed,
    password_confirmation: body.password_confirmation
  };

  match create_user_data(db.clone(), new_user).await {
    Ok(user) => {
      let converted_data = convert_struct_to_value(user);
      let body = ResponseDataEnum::SingleValue(converted_data);

      Ok(body)
    },
    Err(err) => Err(err)
  }
}