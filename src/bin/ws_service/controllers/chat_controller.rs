use validator::Validate;
use serde_json::{json, Value};
use actix_web::{web, Responder};

use crate::{
  services::{
    chat_service::*,
    room_service::*
  },
  schemas::chat_schema::*,
  structs::main_struct::WSAppState
};

use scholarmate_api::{
  enums::{
    error_enum::ErrorEnum,
    response_enum::ResponseDataEnum
  },
  helpers::response::create_response
};

#[doc = "Get chat data"]
pub async fn get_chat(state: web::Data<WSAppState>, body: web::Query<GeneralChatSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match room_show_by_user_id_service(state.db.clone(), body.uid.to_owned()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully retrieved chat data"),
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

#[doc = "Add chat data"]
pub async fn add_chat(state: web::Data<WSAppState>, body: web::Json<DetailChatSchema>) -> impl Responder {
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  let members = format!("{},{}", body.sender, body.reciver);

  match room_store_service(state.db.clone(), body.name.to_owned(), members).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully created chat data"),
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

#[doc = "Find chat data"]
pub async fn find_chat(state: web::Data<WSAppState>, body: web::Query<GeneralChatSchema>, path: web::Path<i32>) -> impl Responder {
  let room_id = path.into_inner();
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match chat_show_service(state.db.clone(), room_id, body.into_inner()).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully retrieved chat data"),
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

#[doc = "Delete chat data"]
pub async fn delete_chat(state: web::Data<WSAppState>, body: web::Json<DetailChatSchema>, path: web::Path<i32>) -> impl Responder {
  let room_id = path.into_inner();
  let validate_form = body.validate();

  if validate_form.is_err() {
    let data = Value::from(validate_form.err().unwrap().to_string());

    return create_response(
      String::from("unprocessable entity"),
      String::from("please fill all fields"),
      ResponseDataEnum::SingleValue(data)
    )
  }

  match room_destroy_service(state.db.clone(), room_id).await {
    Ok(data) => {
      create_response(
        String::from("success"),
        String::from("successfully deleted chat data"),
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