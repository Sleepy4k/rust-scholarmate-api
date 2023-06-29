use std::env;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{
  decode,
  encode,
  Header,
  Validation,
  EncodingKey,
  DecodingKey,
};

#[doc = "Token struct"]
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenStruct {
  pub id: i32,
  pub role: String,
  pub email: String,
  pub iat: u64,
  pub exp: u64,
}

impl TokenStruct {
  pub fn from_jwt(token: &str) -> Result<TokenStruct, String> {
    let jwt_secret = env::var("JWT_TOKEN_SECRET").unwrap_or("your_secret_token".to_owned());
    let key = DecodingKey::from_secret(jwt_secret.as_ref());
    let validate = Validation::default();

    let decode_token = decode::<TokenStruct>(
      token,
      &key,
      &validate
    );

    match decode_token {
      Ok(token) => Ok(token.claims),
      Err(_) => Err("Token is invalid".to_string())
    }
  }

  pub fn to_jwt(self) -> String {
    let jwt_secret = env::var("JWT_TOKEN_SECRET").unwrap_or("your_secret_token".to_owned());
    let key = EncodingKey::from_secret(jwt_secret.as_ref());
    let header = Header::default();

    let encode_token = encode(&header, &self, &key).unwrap_or(String::new());

    encode_token
  }
}