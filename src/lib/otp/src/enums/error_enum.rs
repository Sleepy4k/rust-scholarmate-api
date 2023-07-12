use err_derive::Error;
use std::time::SystemTimeError;
use data_encoding::DecodeError;

#[doc = "Error enum for OTP service"]
#[derive(Debug, Error)]
pub enum OtpError {
  #[error(display="invalid time provided")]
  InvalidTimeError(#[error(source)] SystemTimeError),
  #[error(display="invalid digest provided: {:?}", _0)]
  InvalidDigest(Vec<u8>),
  #[error(display="invalid secret provided")]
  InvalidSecret(#[error(source)] DecodeError)
}