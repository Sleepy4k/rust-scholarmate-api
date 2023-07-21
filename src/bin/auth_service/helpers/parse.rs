use regex::Regex;
use rand::distributions::{Alphanumeric, DistString};

#[doc = "Replace number with X"]
pub fn replace_number_with_string(input:&str) -> String {
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
pub fn generate_secret_key(secret: &str) -> String {
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
