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