use serde_json::{Value, json};
use actix_web::{HttpResponse, http::StatusCode};

use crate::{
  helpers::parse::slugify,
  structs::response_struct::*,
  enums::response_enum::ResponseDataEnum
};

#[doc = "Create status code for response"]
pub fn create_response_code(status: String) -> StatusCode {
  let code = slugify(status.as_str(), "_").to_lowercase();

  match code.as_str() {
    "success" => StatusCode::OK,
    "created" => StatusCode::CREATED,
    "accepted" => StatusCode::ACCEPTED,
    "no_content" => StatusCode::NO_CONTENT,
    "bad_request" => StatusCode::BAD_REQUEST,
    "unauthorized" => StatusCode::UNAUTHORIZED,
    "forbidden" => StatusCode::FORBIDDEN,
    "not_found" => StatusCode::NOT_FOUND,
    "method_not_allowed" => StatusCode::METHOD_NOT_ALLOWED,
    "not_acceptable" => StatusCode::NOT_ACCEPTABLE,
    "request_timeout" => StatusCode::REQUEST_TIMEOUT,
    "conflict" => StatusCode::CONFLICT,
    "unprocessable_entity" => StatusCode::UNPROCESSABLE_ENTITY,
    "internal_server_error" => StatusCode::INTERNAL_SERVER_ERROR,
    "service_unavailable" => StatusCode::SERVICE_UNAVAILABLE,
    _ => StatusCode::INTERNAL_SERVER_ERROR
  }
}

#[doc = "Create a response template"]
pub fn create_response(status: String, message: String, data: ResponseDataEnum) -> HttpResponse {
  let code = create_response_code(status.to_owned());
  let body = ResponseStruct {
    status,
    message,
    data: data.get_value()
  };

  HttpResponse::build(code)
    .content_type("application/json")
    .json(body)
}

#[doc = "Create a response template with token"]
pub fn create_response_with_token(status: String, message: String, data: ResponseDataEnum, token: String) -> HttpResponse {
  let code = create_response_code(status.to_owned());
  let body = ResponseStructWithToken {
    status,
    token,
    message,
    data: data.get_value()
  };

  HttpResponse::build(code)
    .content_type("application/json")
    .json(body)
}

#[doc = "Create a response template for binary"]
pub fn create_response_binary(status: String, data: ResponseDataEnum, disposition: String, content_type: String) -> HttpResponse {
  let code = create_response_code(status.to_owned());
  let body = data.get_value();

  HttpResponse::build(code)
    .insert_header(("Access-Control-Expose-Headers", "Content-Disposition"))
    .insert_header(("Content-Disposition", disposition))
    .insert_header(("Content-Type", content_type))
    .json(body)
}

#[doc = "Set status code for response"]
#[deprecated(note = "Please use create_response_code instead")]
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
#[deprecated(note = "Please use create_response instead")]
pub fn response_json(status: String, message: String, data: Vec<Value>) -> HttpResponse {
  // init response
  let code = setup_code(status.to_owned());

  // set response body
  let body = json!({
    "status": status,
    "message": message,
    "data": data
  });

  // return response
  HttpResponse::build(code)
    .content_type("application/json")
    .json(body)
}

#[doc = "Create a response template with cookie"]
#[deprecated(note = "Please use create_response_with_token instead")]
pub fn response_json_with_cookie(status: String, message: String, data: Vec<Value>, token: String) -> HttpResponse {
  // init response
  let code = setup_code(status.to_owned());

  // set response body
  let body = json!({
    "status": status,
    "token": token,
    "message": message,
    "data": data
  });

  // return response
  HttpResponse::build(code)
    .content_type("application/json")
    .json(body)
}

#[doc = "Create a response template for file"]
#[deprecated(note = "Please use create_response_binary instead")]
pub fn response_file(status: String, body: Vec<u8>, disposition: String, content_type: String) -> HttpResponse {
  // init response
  let code = setup_code(status.to_owned());

  // return response
  HttpResponse::build(code)
    .append_header(("Access-Control-Expose-Headers", "Content-Disposition"))
    .append_header(("Content-Disposition", disposition))
    .append_header(("Content-Type", content_type))
    .body(body)
}
