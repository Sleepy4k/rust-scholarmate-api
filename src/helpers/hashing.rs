use argon2::{self, Config};

#[doc = "Hash a password"]
pub fn hash_password(password: &str, salt: &[u8]) -> String {
  let config = Config::default();
  
  match argon2::hash_encoded(password.as_bytes(), salt, &config) {
    Ok(hash) => hash,
    Err(err) => {
      println!("Error hashing password: {}", err);
      String::new()
    }
  }
}

#[doc = "Verify a password against a hash"]
pub fn verify_password(password: &str, hashed_password: &str) -> bool {
  argon2::verify_encoded(hashed_password, password.as_bytes()).unwrap_or(false)
}