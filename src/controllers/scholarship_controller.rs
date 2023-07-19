use validator::Validate;
use serde_json::{Value, json};
use actix_web::{web, Responder};

use crate::{
  schemas::scholarship_schema::*,
  structs::main_struct::AppState,
  services::scholarship_service::*,
  helpers::response::create_response,
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

#[doc = "Display a listing of the resource."]
pub async fn scholarship_index_controller(state: web::Data<AppState>) -> impl Responder {
  match scholarship_index_service(state.db.to_owned()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully retrieved scholarship data"),
        data
      )
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(message) => {
          create_response(
            String::from("unprocessable entity"),
            message,
            ResponseDataEnum::SingleValue(json!({}))
          )
        },
        _ => {
          create_response(
            String::from("internal server error"),
            err.get_error(),
            ResponseDataEnum::SingleValue(json!({}))
          )
        }
      }
    }
  }
}

#[doc = "Store a newly created resource in storage."]
pub async fn scholarship_store_controller(state: web::Data<AppState>, body: web::Json<ScholarshipSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match scholarship_store_service(state.db.to_owned(), body.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully created scholarship data"),
        data
      )
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(message) => {
          create_response(
            String::from("unprocessable entity"),
            message,
            ResponseDataEnum::SingleValue(json!({}))
          )
        },
        _ => {
          create_response(
            String::from("internal server error"),
            err.get_error(),
            ResponseDataEnum::SingleValue(json!({}))
          )
        }
      }
    }
  }
}

#[doc = "Display the specified resource."]
pub async fn scholarship_show_controller(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  match scholarship_show_service(state.db.to_owned(), path.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully retrieved scholarship data"),
        data
      )
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(message) => {
          create_response(
            String::from("unprocessable entity"),
            message,
            ResponseDataEnum::SingleValue(json!({}))
          )
        },
        _ => {
          create_response(
            String::from("internal server error"),
            err.get_error(),
            ResponseDataEnum::SingleValue(json!({}))
          )
        }
      }
    }
  }
}

#[doc = "Update the specified resource in storage."]
pub async fn scholarship_update_controller(state: web::Data<AppState>, body: web::Json<ScholarshipSchema>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match scholarship_update_service(state.db.to_owned(), id, body.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully updated scholarship data"),
        data
      )
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(message) => {
          create_response(
            String::from("unprocessable entity"),
            message,
            ResponseDataEnum::SingleValue(json!({}))
          )
        },
        _ => {
          create_response(
            String::from("internal server error"),
            err.get_error(),
            ResponseDataEnum::SingleValue(json!({}))
          )
        }
      }
    }
  }
}

#[doc = "Remove the specified resource from storage."]
pub async fn scholarship_destroy_controller(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  match scholarship_destroy_service(state.db.to_owned(), path.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully deleted scholarship data"),
        data
      )
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(message) => {
          create_response(
            String::from("unprocessable entity"),
            message,
            ResponseDataEnum::SingleValue(json!({}))
          )
        },
        _ => {
          create_response(
            String::from("internal server error"),
            err.get_error(),
            ResponseDataEnum::SingleValue(json!({}))
          )
        }
      }
    }
  }
}
