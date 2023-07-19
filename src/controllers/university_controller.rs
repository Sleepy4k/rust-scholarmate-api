use validator::Validate;
use serde_json::{Value, json};
use actix_web::{web, Responder};

use crate::{
  schemas::university_schema::*,
  structs::main_struct::AppState,
  services::university_service::*,
  helpers::response::create_response,
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  }
};

#[doc = "Display a listing of the resource."]
pub async fn university_index_controller(state: web::Data<AppState>) -> impl Responder {
  match university_index_service(state.db.to_owned()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully retrieved university data"),
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
pub async fn university_store_controller(state: web::Data<AppState>, body: web::Json<UniversitySchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match university_store_service(state.db.to_owned(), body.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully created university data"),
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
pub async fn university_show_controller(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  match university_show_service(state.db.to_owned(), path.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully retrieved university data"),
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
pub async fn university_update_controller(state: web::Data<AppState>, body: web::Json<UniversitySchema>, path: web::Path<i32>) -> impl Responder {
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

  match university_update_service(state.db.to_owned(), id, body.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully updated university data"),
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
pub async fn university_destroy_controller(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  match university_destroy_service(state.db.to_owned(), path.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully deleted university data"),
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