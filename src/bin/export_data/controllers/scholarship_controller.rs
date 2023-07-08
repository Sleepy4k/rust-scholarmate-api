use std::path::PathBuf;
use actix_web::{web, Responder};

use crate::{
  schemas::global_schema::GeneralSchema,
  helpers::{
    date::*,
    build_csv::*,
    build_excel::*,
    parse::build_data
  }
};

use scholarmate_api::{
  structs::main_struct::AppState,
  helpers::response::response_file,
  repositories::{
    scholarship_repository::fetch_scholarship_data,
    translate_repository::fetch_translate_data_by_table
  }
};

const TABLE_NAME: &str = "scholarships";

#[doc = "Export scholarship data to csv file"]
pub async fn scholarship_export_csv(state: web::Data<AppState>, body: web::Json<GeneralSchema>) -> impl Responder {
  let table = TABLE_NAME.to_string();
  let data = fetch_scholarship_data(state.db.clone()).await;
  let result = build_data(data).await.unwrap_or(Vec::new());
  let fields = fetch_translate_data_by_table(state.db.clone(), table.to_owned()).await;
  
  let formatted_datetime = get_current_time();
  let file_name = format!("{}-{}.csv", table, formatted_datetime);
  let path: PathBuf = file_name.parse().unwrap();

  let last_res = build_csv_file(
    (result, fields),
    body.fields.to_owned(),
    path
  ).await.unwrap_or(Vec::new());

  response_file(
    String::from("success"),
    last_res,
    format!("attachment; filename={}", file_name),
    String::from("text/csv"),
  )
}

#[doc = "Export scholarship data to excel file"]
pub async fn scholarship_export_excel(state: web::Data<AppState>, body: web::Json<GeneralSchema>) -> impl Responder {
  let table = TABLE_NAME.to_string();
  let data = fetch_scholarship_data(state.db.clone()).await;
  let result = build_data(data).await.unwrap_or(Vec::new());
  let fields = fetch_translate_data_by_table(state.db.clone(), table.to_owned()).await;
  
  let formatted_datetime = get_current_time();
  let file_name = format!("{}-{}.xlsx", table, formatted_datetime);
  let path: PathBuf = file_name.parse().unwrap();

  let last_res = build_excel_file(
    (result, fields),
    body.fields.to_owned(),
    path
  ).await.unwrap_or(Vec::new());

  response_file(
    String::from("success"),
    last_res,
    format!("attachment; filename={}", file_name),
    String::from("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
  )
}