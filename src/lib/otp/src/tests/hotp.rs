use pretty_assertions::{assert_eq, assert_ne};

use crate::services::hotp_service::make_hmac_otp;

#[test]
pub fn test_create_hmac_otp() {
  let secret = "AkuCumaAnakMamah";
  let build = make_hmac_otp(secret, 0).unwrap();
  let build_again = make_hmac_otp(secret, 0).unwrap();

  let token = "sarahpalastring";

  println!("token: {:?}", token.as_bytes());

  assert_eq!(build, build_again);
}

#[test]
pub fn test_check_hmac_otp() {
  let secret = "AkuCumaAnakMamah";
  let build = make_hmac_otp(secret, 0).unwrap();
  let build_again = make_hmac_otp(secret, 5).unwrap();

  assert_ne!(build, build_again);
}