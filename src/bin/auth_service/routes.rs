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

    // auth route
    .route("/login", web::post().to(login))
    .route("/register", web::post().to(register))
    .route("/logout", web::post().to(logout))

    // fallback route
    .default_service(web::route().to(fallback));
}