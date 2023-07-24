use sqlx::{Pool, Postgres};

use crate::{
  schemas::GeneralSchema,
  services::main_service::export_data_logic
};

use scholarmate_api::{
  enums::error_enum::ErrorEnum,
  services::scholarship_service::scholarship_index_service
};

#[doc = "handle scholarship export data logic"]
pub async fn handle_scholarship_export(db: Pool<Postgres>, body: GeneralSchema, table: String, ext: String) -> anyhow::Result<(Vec<u8>, String, String), ErrorEnum> {
  match scholarship_index_service(db.clone()).await {
    Ok(data_enum) => {
      match export_data_logic(db.clone(), body, data_enum, table, ext).await {
        Ok((last_res, mime_type, file_name)) => Ok((last_res, mime_type, file_name)),
        Err(err) => match err {
          ErrorEnum::CustomError(_) => Ok((vec![], String::from(""), String::from(""))),
          _ => Err(err)
        }
      }
    },
    Err(err) => match err {
      ErrorEnum::CustomError(_) => Ok((vec![], String::from(""), String::from(""))),
      _ => Err(err)
    }
  }
}