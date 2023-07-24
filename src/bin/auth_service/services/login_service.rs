use serde_json::json;
use sqlx::{Pool, Postgres};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
  schemas::login_schema::*,
  repositories::auth_repository::*,
  models::auth_model::DetailUserModel
};

use scholarmate_api::{
  structs::auth_struct::TokenStruct,
  services::student_service::student_show_by_exist_column_service,
  helpers::{
    hashing::verify_password,
    parse::convert_struct_to_value
  },
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

#[doc = "Display a listing of the resource."]
pub async fn login_index_service(db: Pool<Postgres>, body: LoginSchema) -> anyhow::Result<(ResponseDataEnum, String), ErrorEnum> {
  match find_user_data_by_email(db.clone(), body.email.to_owned()).await {
    Ok(user) => {
      if !verify_password(body.password.as_str(), &user.password) {
        return Err(ErrorEnum::CustomError(String::from("email or password is wrong")));
      }

      if !user.verified {
        return Err(ErrorEnum::CustomError(String::from("please verify your account first")));
      }

      let token_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
  
      let claims = TokenStruct {
        id: user.id,
        email: user.email.clone(),
        role: user.role.clone(),
        iat: token_time,
        exp: token_time.saturating_add(604800),
      };

      let token = claims.to_jwt();

      let student = match student_show_by_exist_column_service(db.clone(), user.email.to_owned(), String::new(), String::new()).await {
        Ok(student) => student.get_value(),
        Err(err) => {
          match err {
            ErrorEnum::CustomError(_) => json!({}),
            _ => return Err(ErrorEnum::InternalServerError)
          }
        }
      };

      let detail_user = DetailUserModel {
        id: user.id,
        email: user.email,
        role: user.role,
        verified: user.verified,
        student
      };

      let converted_data = convert_struct_to_value(detail_user);
      let body = ResponseDataEnum::SingleValue(converted_data);

      Ok((body, token))
    },
    Err(err) => Err(err)
  }
}