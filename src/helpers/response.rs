use serde_json::Value;
use actix_web::{HttpResponse, http::StatusCode};

use crate::structs::response_struct::*;

#[doc = "Set status code for response"]
fn setup_code(status: String) -> StatusCode {
  // init response
  let mut code = StatusCode::ACCEPTED;
  let stats = status.to_owned().to_lowercase();

  // set status code
  if stats == "success" {
    code = StatusCode::OK;
  } else if stats == "error" {
    code = StatusCode::INTERNAL_SERVER_ERROR;
  } else if stats == "failed" {
    code = StatusCode::BAD_REQUEST;
  } else if stats == "unauthorize" {
    code = StatusCode::UNAUTHORIZED;
  } else if stats == "forbidden" {
    code = StatusCode::FORBIDDEN;
  } else if stats == "not found" {
    code = StatusCode::NOT_FOUND;
  }

  code
}

#[doc = "Create a response template"]
pub fn response_json(status: String, message: String, data: Vec<Value>) -> HttpResponse {
  // init response
  let code = setup_code(status.to_owned());

  // set response body
  let body = ResponseStruct {
    status,
    message,
    data,
  };

  // return response
  HttpResponse::build(code)
    .content_type("application/json")
    .json(body)
}

#[doc = "Create a response template for file"]
pub fn response_file(status: String, body: Vec<u8>, disposition: String, content_type: String) -> HttpResponse {
  // init response
  let code = setup_code(status.to_owned());

  // return response
  HttpResponse::build(code)
    .append_header(("Content-Disposition", disposition))
    .append_header(("Content-Type", content_type))
    .body(body)
}

#[doc = "Create a response template for file"]
pub fn response_file2(status: String, body: String, disposition: String, content_type: String) -> HttpResponse {
  // init response
  let code = setup_code(status.to_owned());

  // return response
  HttpResponse::build(code)
    .append_header(("Content-Disposition", disposition))
    .append_header(("Content-Type", content_type))
    .json(body)
}

#[doc = "Create a response template with cookie"]
pub fn response_json_with_cookie(status: String, message: String, data: Vec<Value>, token: String) -> HttpResponse {
  // init response
  let code = setup_code(status.to_owned());

  // set response body
  let body = ResponseCookieStruct {
    status,
    message,
    data,
    token,
  };

  // return response
  HttpResponse::build(code)
    .content_type("application/json")
    .json(body)
}