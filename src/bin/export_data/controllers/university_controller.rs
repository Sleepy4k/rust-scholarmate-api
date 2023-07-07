use chrono::Local;
use serde_json::Value;
use actix_web::{web, Responder};

use crate::helpers::{build_csv::*, build_excel::*};
use scholarmate_api::{
  structs::main_struct::AppState,
  helpers::{
    response::{response_file, response_file2},
    parse::{map_get, to_str},
  },
  repositories::{
    university_repository::fetch_university_data,
    translate_repository::fetch_translate_data_by_table
  }
};

pub async fn export_excel(state: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
  let fields = fetch_translate_data_by_table(state.db.clone(), to_str(map_get("table", body.to_owned()))).await;
  let data = fetch_university_data(state.db.clone()).await;
  let result = build_excel_data(data).await.unwrap_or(Vec::new());

  let last_res = build_excel_file(
    (result, fields),
    map_get("fields", body.to_owned())
  ).await.unwrap_or(Vec::new());

  let current_datetime = Local::now();
  let formatted_datetime = current_datetime.format("%H%M%S").to_string();

  response_file(
    String::from("success"),
    last_res,
    String::from("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
    format!("form-data; filename=universities-{}.xlsx", formatted_datetime),
  )
}

pub async fn export_csv(state: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
  let fields = fetch_translate_data_by_table(state.db.clone(), to_str(map_get("table", body.to_owned()))).await;
  let data = fetch_university_data(state.db.clone()).await;

  let formatted_data = data
    .into_iter()
    .map(|x| {
      build_csv_file(x, fields.clone())
    })
    .collect::<Vec<_>>();

  let data = write_csv_file(formatted_data).await;

  let current_datetime = Local::now();
  let formatted_datetime = current_datetime.format("%H%M%S").to_string();

  response_file2(
    String::from("success"),
    data,
    String::from("text/csv"),
    format!("form-data; filename=universities-{}.csv", formatted_datetime),
  )
}