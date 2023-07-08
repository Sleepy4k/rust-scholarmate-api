use actix_web::web;

use crate::controllers::*;

#[doc = "Set routes for application"]
pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    // welcome route
    .route("/", web::route().to(welcome))

    // auth route
    .route("/login", web::post().to(login))
    .route("/register", web::post().to(register))
    .route("/logout", web::post().to(logout))

    // user route
    .route("/user", web::get().to(get_user))

    // forum route
    .route("/forum/{id}", web::get().to(get_forum))

    // university route
    .service(
      web::scope("/university")
        .route("", web::get().to(get_university))
        .route("", web::post().to(add_university))
        .route("/{id}", web::get().to(find_university))
        .route("/{id}", web::put().to(update_university))
        .route("/{id}", web::delete().to(delete_university))
    )

    // scholarship route
    .service(
      web::scope("/scholarship")
        .route("", web::get().to(get_scholarship))
        .route("", web::post().to(add_scholarship))
        .route("/{id}", web::get().to(find_scholarship))
        .route("/{id}", web::put().to(update_scholarship))
        .route("/{id}", web::delete().to(delete_scholarship))
    )

    // student route
    .service(
      web::scope("/student")
        .route("", web::get().to(get_student))
        .route("", web::post().to(add_student))
        .route("/{id}", web::get().to(find_student))
        .route("/{id}", web::put().to(update_student))
        .route("/{id}", web::delete().to(delete_student))
    )

    // application route
    .service(
      web::scope("/application")
        .route("", web::get().to(get_application))
        .route("", web::post().to(add_application))
        .route("/{id}", web::get().to(get_my_application))
    )

    // apply route
    .service(
      web::scope("/apply")
        .route("", web::post().to(add_student))
        .route("/{id}", web::put().to(update_student))
    )

    // fallback route
    .default_service(web::route().to(fallback));
}