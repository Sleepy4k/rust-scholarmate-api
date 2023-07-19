use validator::Validate;
use serde_json::{Value, json};
use actix_web::{web, Responder};

use crate::{
  schemas::application_schema::*,
  structs::main_struct::AppState,
  helpers::response::create_response,
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  },
  services::{
    university_service::*,
    application_service::*,
    scholarship_service::*
  }
};

#[doc = "Display a listing of the resource."]
pub async fn application_index_controller(state: web::Data<AppState>) -> impl Responder {
  match application_get_service(state.db.to_owned()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully retrieved application data"),
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
pub async fn application_store_controller(state: web::Data<AppState>, body: web::Json<ApplicationSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("Please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match university_show_service(state.db.to_owned(), body.univ_id).await {
    Ok(response) => {
      let data = response.get_value();

      if data["major"] != body.major {
        return create_response(
          String::from("unprocessable entity"),
          String::from("major not found"),
          ResponseDataEnum::SingleValue(json!({}))
        )
      } else if data["quantity"] == 0 {
        return create_response(
          String::from("unprocessable entity"),
          String::from("university quota is full"),
          ResponseDataEnum::SingleValue(json!({}))
        )
      } else {
        ()
      }
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(message) => {
          return create_response(
            String::from("unprocessable entity"),
            message,
            ResponseDataEnum::SingleValue(json!({}))
          )
        },
        _ => {
          return create_response(
            String::from("internal server error"),
            err.get_error(),
            ResponseDataEnum::SingleValue(json!({}))
          )
        }
      }
    }
  };

  match scholarship_show_service(state.db.to_owned(), body.scholarship_id).await {
    Ok(response) => {
      let data = response.get_value();

      if data["quantity"] == 0 {
        return create_response(
          String::from("unprocessable entity"),
          String::from("scholarship quota is full"),
          ResponseDataEnum::SingleValue(json!({}))
        )
      } else {
        ()
      }
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(message) => {
          return create_response(
            String::from("unprocessable entity"),
            message,
            ResponseDataEnum::SingleValue(json!({}))
          )
        },
        _ => {
          return create_response(
            String::from("internal server error"),
            err.get_error(),
            ResponseDataEnum::SingleValue(json!({}))
          )
        }
      }
    }
  };

  match application_store_service(state.db.to_owned(), body.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully created application data"),
        data
      )
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(message) => {
          return create_response(
            String::from("unprocessable entity"),
            message,
            ResponseDataEnum::SingleValue(json!({}))
          )
        },
        _ => {
          return create_response(
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
pub async fn application_show_controller(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  match application_show_service(state.db.to_owned(), path.into_inner()).await {
    Ok(data) => {
      if data.is_empty() {
        return create_response(
          String::from("not found"),
          String::from("application data not found"),
          ResponseDataEnum::SingleValue(json!({}))
        );
      }

      create_response(
        String::from("success"),
        String::from("successfully retrieved application data"),
        data
      )
    },
    Err(err) => {
      match err {
        ErrorEnum::CustomError(message) => {
          return create_response(
            String::from("unprocessable entity"),
            message,
            ResponseDataEnum::SingleValue(json!({}))
          )
        },
        _ => {
          return create_response(
            String::from("internal server error"),
            err.get_error(),
            ResponseDataEnum::SingleValue(json!({}))
          )
        }
      }
    }
  }
}
