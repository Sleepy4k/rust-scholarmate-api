use serde::{Serialize, Deserialize};

#[doc = "Response enum for error"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorEnum {
  DatabaseError,
  NotFoundError,
  InternalServerError,
  BadRequestError,
  UnauthorizedError,
  ForbiddenError,
  UnprocessableEntityError,
  ConflictError,
  ValidationError,
  CustomError(String),
}

impl ErrorEnum {
  pub fn get_error(&self) -> String {
    match self {
      ErrorEnum::DatabaseError => String::from("database error"),
      ErrorEnum::NotFoundError => String::from("not found error"),
      ErrorEnum::InternalServerError => String::from("internal server error"),
      ErrorEnum::BadRequestError => String::from("bad request error"),
      ErrorEnum::UnauthorizedError => String::from("unauthorized error"),
      ErrorEnum::ForbiddenError => String::from("forbidden error"),
      ErrorEnum::UnprocessableEntityError => String::from("unprocessable entity error"),
      ErrorEnum::ConflictError => String::from("conflict error"),
      ErrorEnum::ValidationError => String::from("validation error"),
      ErrorEnum::CustomError(err) => err.to_string(),
    }
  }
}