use rand::Rng;
use chrono::prelude::*;

#[doc = "generate token from string"]
pub async fn generate_token(data: String) -> Vec<u32> {
  let mut token = Vec::new();
  let mut rng = rand::thread_rng();
  let mut counter = 0;

  while counter < 6 {
    let random_number = rng.gen_range(0..data.len());
    let random_char = data.chars().nth(random_number).unwrap() as u32;

    token.push(random_char);

    counter += 1;
  }

  let local_time = Local::now();
  let date_str = local_time.format("%Y%m%d").to_string();
  let parsed = date_str.parse::<u32>().expect("Failed to parse date as u32");
  
  token.push(parsed);

  token
}