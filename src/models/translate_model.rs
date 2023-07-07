use serde::{Serialize, Deserialize};

#[doc = "Translate model"]
#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateModel {
  pub id: i32,
  pub table_name: String,
  pub column_name: String,
  pub language: String,
}

#[doc = "Detail translate model"]
#[derive(Debug, Serialize, Deserialize)]
pub struct DetailTranslateModel {
  pub column_name: String,
  pub language: String,
}