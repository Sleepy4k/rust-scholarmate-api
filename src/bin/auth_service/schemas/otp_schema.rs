use validator::Validate;
use serde::{Serialize, Deserialize};

#[doc = "OTP schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct OTPSchema {
  #[validate(length(min = 1, max = 255, message = "email is required and must be less than 255 characters"), email(message = "email must be a valid email"))]
  pub email: String,
}

#[doc = "Detail OTP schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct DetailOTPSchema {
  #[validate(length(min = 1, max = 255, message = "email is required and must be less than 255 characters"), email(message = "email must be a valid email"))]
  pub email: String,
  #[validate(length(min = 1, max = 255, message = "token is required and must be less than 255 characters"))]
  pub token: String,
}

#[doc = "OTP verification schema"]
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct OTPVerificationSchema {
  #[validate(length(min = 1, max = 255, message = "email is required and must be less than 255 characters"), email(message = "email must be a valid email"))]
  pub email: String,
  #[validate(length(min = 1, max = 255, message = "otp is required and must be less than 255 characters"))]
  pub token: String,
  #[validate(length(min = 1, max = 255, message = "otp is required and must be less than 255 characters"))]
  pub otp: String,
}