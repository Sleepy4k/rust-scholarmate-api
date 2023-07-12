use tokenizers::tokenizer::{Result, Tokenizer};

#[doc = "generate token from string"]
pub async fn generate_token(identifier: &str, data: String) -> Result<Vec<u32>> {
  let tokenizer = Tokenizer::from_pretrained(identifier, None)?;
  let tokens = tokenizer.encode(data, false)?;

  Ok(tokens.get_ids().to_vec())
}

#[doc = "decode token from Vec<u32>"]
pub async fn decode_token(identifier: &str, data: Vec<u32>) -> Result<String> {
  let tokenizer = Tokenizer::from_pretrained(identifier, None)?;
  let tokens = tokenizer.decode(data, false)?;

  Ok(tokens)
}