use actix_web::web::{self};

use crate::controllers::*;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    // welcome route
    .route("/", web::route().to(welcome))

    // auth route
    .route("/login", web::post().to(login))
    .route("/register", web::post().to(register))
    .route("/logout", web::post().to(logout))

    // university route
    .route("/university", web::get().to(get_university))
    .route("/university", web::post().to(add_university))
    .route("/university/{id}", web::get().to(find_university))
    .route("/university/{id}", web::put().to(update_university))
    .route("/university/{id}", web::delete().to(delete_university))

    // scholarship route
    .route("/scholarship", web::get().to(get_scholarship))
    .route("/scholarship", web::post().to(add_scholarship))
    .route("/scholarship/{id}", web::get().to(find_scholarship))
    .route("/scholarship/{id}", web::put().to(update_scholarship))
    .route("/scholarship/{id}", web::delete().to(delete_scholarship))

    // student route
    .route("/student", web::get().to(get_student))
    .route("/student", web::post().to(add_student))
    .route("/student/{id}", web::get().to(find_student))
    .route("/student/{id}", web::put().to(update_student))
    .route("/student/{id}", web::delete().to(delete_student))

    // application route
    .route("/application", web::get().to(get_application))
    .route("/application/{id}", web::get().to(get_my_application))
    .route("/application", web::post().to(add_application))

    // forum route
    .route("/forum/{id}", web::get().to(get_forum))

    // apply route
    .route("/apply", web::post().to(post_apply))
    .route("/apply/{id}", web::put().to(put_apply))

    // user route
    .route("/user", web::get().to(get_user))

    // fallback route
    .default_service(web::route().to(fallback));
}