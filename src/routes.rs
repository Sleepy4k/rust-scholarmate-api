use actix_web::web;

use crate::controllers::*;

#[doc = "Set routes for application"]
pub fn service_config(cfg: &mut web::ServiceConfig) {
  cfg
    // welcome route
    .route("/", web::route().to(welcome))

    // user route
    .route("/user", web::get().to(user_index_controller))

    // forum route
    .route("/forum/{id}", web::get().to(forum_show_controller))

    // university route
    .service(
      web::scope("/university")
        .route("", web::get().to(university_index_controller))
        .route("", web::post().to(university_store_controller))
        .route("/{id}", web::get().to(university_show_controller))
        .route("/{id}", web::put().to(university_update_controller))
        .route("/{id}", web::delete().to(university_destroy_controller))
    )

    // scholarship route
    .service(
      web::scope("/scholarship")
        .route("", web::get().to(scholarship_index_controller))
        .route("", web::post().to(scholarship_store_controller))
        .route("/{id}", web::get().to(scholarship_show_controller))
        .route("/{id}", web::put().to(scholarship_update_controller))
        .route("/{id}", web::delete().to(scholarship_destroy_controller))
    )

    // student route
    .service(
      web::scope("/student")
        .route("", web::get().to(student_index_controller))
        .route("", web::post().to(student_store_controller))
        .route("/{id}", web::get().to(student_show_controller))
        .route("/{id}", web::put().to(student_update_controller))
        .route("/{id}", web::delete().to(student_destroy_controller))
    )

    // application route
    .service(
      web::scope("/application")
        .route("", web::get().to(application_index_controller))
        .route("", web::post().to(application_store_controller))
        .route("/{id}", web::get().to(application_show_controller))
    )

    // apply route
    .service(
      web::scope("/apply")
        .route("", web::post().to(student_store_controller))
        .route("/{id}", web::put().to(student_update_controller))
    )

    // fallback route
    .default_service(web::route().to(fallback));
}