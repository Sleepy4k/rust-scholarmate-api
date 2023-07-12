use std::time::SystemTime;

use crate::{
  enums::error_enum::OtpError,
  services::hotp_service::make_hmac_otp
};

#[doc = "Make time OTP from secret, time step, skew, and time"]
fn make_time_otp_helper(secret: &str, time_step: u64, skew: i64, time: u64) -> Result<u32, OtpError> {
  let counter = ((time as i64 + skew) as u64) / time_step;

  make_hmac_otp(secret, counter)
}

#[doc = "Make time OTP from secret, time step, and skew"]
pub fn make_time_otp(secret: &str, time_step: u64, skew: i64) -> Result<u32, OtpError> {
  let now = SystemTime::now();
  let time_since_epoch = now.duration_since(SystemTime::UNIX_EPOCH)?;

  match make_time_otp_helper(secret, time_step, skew, time_since_epoch.as_secs() ) {
    Ok(d) => Ok(d),
    Err(err) => return Err(err)
  }
}