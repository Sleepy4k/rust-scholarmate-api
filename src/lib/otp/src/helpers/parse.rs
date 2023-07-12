use ring::hmac;
use regex::Regex;
use std::convert::TryInto;
use data_encoding::{BASE32_NOPAD, DecodeError};
use rand::distributions::{Alphanumeric, DistString};

use crate::enums::error_enum::OtpError;

#[doc = "Replace number with X"]
fn replace_number_with_string(input:&str) -> String {
  let re = Regex::new(r"\d+").unwrap();
  let replaced = re.replace_all(input, |caps: &regex::Captures| {
    let matched = caps.get(0).unwrap().as_str();
    let matched_length = matched.len();
    let mut replaced = String::new();

    for _ in 0..matched_length {
      replaced.push('X');
    }

    replaced
  });

  replaced.into_owned()
}

#[doc = "Handle u8 for secret key"]
fn generate_secret_key(secret: &str) -> String {
  let secret_key;
  let secret = secret.replace(" ", "");
  let secret_length = secret.len();

  if secret_length < 16 {
    let mut rng = rand::thread_rng();
    let missing_chars = 16 - secret_length;
    let random_chars = Alphanumeric.sample_string(&mut rng, missing_chars);

    secret_key = format!("{}{}", secret, random_chars);
  } else if secret_length > 16 {
    secret_key = secret[..16].to_string();
  } else {
    secret_key = secret.to_string();
  }

  let replaced_key = replace_number_with_string(&secret_key);

  replaced_key.to_uppercase()
}

#[doc = "Decode secret from base32 to bytes"]
pub fn decode_secret(secret: &str) -> Result<Vec<u8>, DecodeError> {
  let secret = replace_number_with_string(secret);
  let secret_key = generate_secret_key(secret.as_str());
  let secret_key = secret_key.as_str();

  BASE32_NOPAD.decode(secret_key.as_bytes())
}

#[doc = "Calculate digest from secret and counter"]
pub fn calc_digest(decoded_secret: &[u8], counter: u64) -> hmac::Tag {
  let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, decoded_secret);

  hmac::sign(&key, &counter.to_be_bytes())
}

#[doc = "Encode digest to OTP"]
pub fn encode_digest(digest: &[u8]) -> Result<u32, OtpError> {
  let offset = match digest.last() {
    Some(x) => *x & 0xf,
    None => return Err(OtpError::InvalidDigest(Vec::from(digest)))
  } as usize;

  let code_bytes: [u8; 4] = match digest[offset..offset+4].try_into() {
    Ok(x) => x,
    Err(_) => return Err(OtpError::InvalidDigest(Vec::from(digest)))
  };

  let code = u32::from_be_bytes(code_bytes);
  
  Ok((code & 0x7fffffff) % 1_000_000)
}