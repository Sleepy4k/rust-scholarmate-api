use pretty_assertions::{assert_eq, assert_ne};

use crate::services::totp_service::make_time_otp;

#[test]
pub fn test_create_time_otp() {
  let secret = "AkuCumaAnakMamah";
  let build = make_time_otp(secret, 60, 0).unwrap();
  let build_again = make_time_otp(secret, 60, 0).unwrap();

  assert_eq!(build, build_again);
}

#[test]
pub fn test_check_time_otp() {
  let secret = "AkuCumaAnakMamah";
  let build = make_time_otp(secret, 3600, 0).unwrap();
  let build_again = make_time_otp(secret, 90, 0).unwrap();

  assert_ne!(build, build_again);
}