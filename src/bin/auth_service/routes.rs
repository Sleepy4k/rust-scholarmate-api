use actix_web::web;

use crate::controllers::*;

use scholarmate_api::controllers::{
  welcome_controller::*,
  fallback_controller::*
};

#[doc = "Set routes for application"]
pub fn service_config(cfg: &mut web::ServiceConfig) {
  cfg
    // welcome route
    .route("/", web::route().to(welcome))

    // auth route
    .route("/login", web::post().to(login_index_controller))
    .route("/register", web::post().to(register_index_controller))
    .route("/logout", web::post().to(logout_index_controller))

    // otp route
    .service(
      web::scope("/otp")
        .route("", web::post().to(add_otp))
        .route("", web::put().to(update_otp))
        .route("", web::delete().to(delete_otp))
        .route("validate", web::post().to(validate_otp))
    )

    // fallback route
    .default_service(web::route().to(fallback));
}