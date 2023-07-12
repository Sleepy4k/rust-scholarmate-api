use crate::{
  enums::error_enum::OtpError,
  helpers::parse::{
    calc_digest,
    decode_secret,
    encode_digest
  }
};

#[doc = "Make HMAC OTP from secret and counter"]
pub fn make_hmac_otp(secret: &str, counter: u64) -> Result<u32, OtpError> {
  let decoded = decode_secret(secret)?;

  encode_digest(calc_digest(decoded.as_slice(), counter).as_ref())
}