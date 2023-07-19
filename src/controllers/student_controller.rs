use validator::Validate;
use serde_json::{Value, json};
use actix_web::{web, Responder};

use crate::{
  schemas::student_schema::*,
  services::student_service::*,
  structs::main_struct::AppState,
  helpers::response::create_response,
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

#[doc = "Display a listing of the resource."]
pub async fn student_index_controller(state: web::Data<AppState>) -> impl Responder {
  match student_index_service(state.db.to_owned()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully retrieved student data"),
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
pub async fn student_store_controller(state: web::Data<AppState>, body: web::Json<StudentSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match student_store_service(state.db.to_owned(), body.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully created student data"),
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
pub async fn student_show_controller(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  match student_show_service(state.db.to_owned(), path.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully retrieved student data"),
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
pub async fn student_update_controller(state: web::Data<AppState>, body: web::Json<StudentSchema>, path: web::Path<i32>) -> impl Responder {
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

  match student_update_service(state.db.to_owned(), id, body.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully updated student data"),
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
pub async fn student_destroy_controller(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  match student_destroy_service(state.db.to_owned(), path.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully deleted student data"),
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