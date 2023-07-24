use validator::Validate;
use serde_json::{Value, json};
use actix_web::{web, Responder};

use crate::{
  services::university_service::*,
  schemas::global_schema::GeneralSchema
};

use scholarmate_api::{
  structs::main_struct::AppState,
  helpers::response::{
    create_response,
    create_response_binary
  },
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

const TABLE_NAME: &str = "universities";

#[doc = "Export university data to csv file"]
pub async fn university_export_csv(state: web::Data<AppState>, body: web::Json<GeneralSchema>) -> impl Responder {
  let validate_form = body.validate();
  let table = TABLE_NAME.to_string();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match handle_university_export(state.db.clone(), body.into_inner(), table.to_owned(), String::from("csv")).await {
    Ok((data, mime_type, file_name)) => create_response_binary(
      String::from("success"),
      data,
      format!("attachment; filename={}", file_name),
      mime_type
    ),
    Err(err) => match err {
      ErrorEnum::CustomError(_) => create_response(
        String::from("unprocessable entity"),
        err.get_error(),
        ResponseDataEnum::SingleValue(json!({}))
      ),
      _ => create_response(
        String::from("internal server error"),
        err.get_error(),
        ResponseDataEnum::SingleValue(json!({}))
      )
    }
  }
}

#[doc = "Export university data to excel file"]
pub async fn university_export_excel(state: web::Data<AppState>, body: web::Json<GeneralSchema>) -> impl Responder {
  let validate_form = body.validate();
  let table = TABLE_NAME.to_string();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match handle_university_export(state.db.clone(), body.into_inner(), table.to_owned(), String::from("xlsx")).await {
    Ok((data, mime_type, file_name)) => create_response_binary(
      String::from("success"),
      data,
      format!("attachment; filename={}", file_name),
      mime_type
    ),
    Err(err) => match err {
      ErrorEnum::CustomError(_) => create_response(
        String::from("unprocessable entity"),
        err.get_error(),
        ResponseDataEnum::SingleValue(json!({}))
      ),
      _ => create_response(
        String::from("internal server error"),
        err.get_error(),
        ResponseDataEnum::SingleValue(json!({}))
      )
    }
  }
}