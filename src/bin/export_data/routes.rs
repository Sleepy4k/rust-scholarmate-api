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

    // university routes
    .service(
      web::scope("/university")
        .route("csv", web::post().to(university_export_csv))
        .route("excel", web::post().to(university_export_excel))
    )

    // student routes
    .service(
      web::scope("/student")
        .route("csv", web::post().to(student_export_csv))
        .route("excel", web::post().to(student_export_excel))
    )

    // user routes
    .service(
      web::scope("/user")
        .route("csv", web::post().to(user_export_csv))
        .route("excel", web::post().to(user_export_excel))
    )

    // scholarship routes
    .service(
      web::scope("/scholarship")
        .route("csv", web::post().to(scholarship_export_csv))
        .route("excel", web::post().to(scholarship_export_excel))
    )

    // fallback route
    .default_service(web::route().to(fallback));
}