use serde::{Serialize, Deserialize};

#[doc = "OTP model"]
#[derive(Debug, Deserialize, Serialize)]
pub struct OTPModel {
  pub id: i32,
  pub token: String,
  pub otp: String,
  pub email: String,
}