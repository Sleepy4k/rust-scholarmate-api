use actix_web::web;

use crate::controllers::*;
use scholarmate_api::controllers::fallback_controller::*;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    // welcome route
    .route("/", web::route().to(welcome))

    // university routes
    .service(
      web::scope("/university")
        .route("excel", web::post().to(export_excel))
        .route("csv", web::post().to(export_csv))
    )

    // fallback route
    .default_service(web::route().to(fallback));
}