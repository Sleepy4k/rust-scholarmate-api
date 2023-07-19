use serde_json::json;
use actix_web::{web, Responder};

use crate::{
  services::forum_service::*,
  structs::main_struct::AppState,
  helpers::response::create_response,
  enums::response_enum::ResponseDataEnum
};

#[doc = "Display the specified resource."]
pub async fn forum_show_controller(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  match forum_show_service(state.db.clone(), path.into_inner()).await {
    Ok(data) => {
      if data.is_empty() {
        return create_response(
          String::from("not found"),
          String::from("forum data not found"),
          ResponseDataEnum::SingleValue(json!({}))
        );
      }

      create_response(
        String::from("success"),
        String::from("successfully retrieved forum data"),
        data
      )
    },
    Err(err) => {
      create_response(
        String::from("internal server error"),
        err.get_error(),
        ResponseDataEnum::SingleValue(json!({}))
      )
    }
  }
}