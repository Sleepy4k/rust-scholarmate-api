use actix_web::web;

use crate::controllers::*;

use scholarmate_api::controllers::{
  welcome_controller::*,
  fallback_controller::*
};

#[doc = "Set routes for application"]
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    // welcome route
    .route("/", web::route().to(welcome))

    // otp route
    .service(
      web::scope("/chat")
        .route("ws", web::get().to(chat_server))
        .route("", web::get().to(get_chat))
        .route("", web::post().to(add_chat))
        .route("{room_id}", web::get().to(find_chat))
        .route("{room_id}", web::delete().to(delete_chat))
    )

    // fallback route
    .default_service(web::route().to(fallback));
}