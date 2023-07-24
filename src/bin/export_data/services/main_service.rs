use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{
  schemas::GeneralSchema,
  helpers::{
    date::*,
    build_csv::*,
    build_excel::*,
    parse::build_data
  }
};

use scholarmate_api::{
  enums::{error_enum::ErrorEnum, response_enum::ResponseDataEnum},
  services::translate_service::translate_show_by_table_name_service
};

#[doc = "handle main export data logic"]
pub async fn export_data_logic(db: Pool<Postgres>, body: GeneralSchema, data: ResponseDataEnum, table: String, ext: String) -> anyhow::Result<(Vec<u8>, String, String), ErrorEnum> {
  let data = data.get_array();
  let result = build_data(data).await.unwrap_or(Vec::new());
  let fields = match translate_show_by_table_name_service(db.clone(), table.to_owned()).await {
    Ok(data_enum) => data_enum.get_array(),
    Err(err) => match err {
      ErrorEnum::CustomError(_) => vec![],
      _ => return Err(err)
    }
  };
  
  let formatted_datetime = get_current_time();
  let file_name = format!("{}-{}.{}", table, formatted_datetime, ext.to_lowercase());
  let path = file_name.parse().unwrap();

  let request_field = match body.fields {
    Some(field) => field,
    None => json!({})
  };

  let last_res = match ext.to_lowercase().as_str() {
    "csv" => build_csv_file((result, fields), request_field, path).await.unwrap_or_default(),
    "xlsx" => build_excel_file((result, fields), request_field, path).await.unwrap_or_default(),
    _ => vec![]
  };

  let mime_type = match ext.to_lowercase().as_str() {
    "csv" => String::from("text/csv"),
    "xlsx" => String::from("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
    _ => String::from("application/octet-stream")
  };

  Ok((last_res, mime_type, file_name))
}