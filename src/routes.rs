use actix_web::web::{self};

use crate::controllers::*;

pub fn config(conf: &mut web::ServiceConfig) {
  conf
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

    // schoolarship route
    .route("/schoolarship", web::get().to(get_schoolarship))
    .route("/schoolarship", web::post().to(add_schoolarship))
    .route("/schoolarship/{id}", web::get().to(find_schoolarship))
    .route("/schoolarship/{id}", web::put().to(update_schoolarship))
    .route("/schoolarship/{id}", web::delete().to(delete_schoolarship))

    // student route
    .route("/student", web::get().to(get_student))
    .route("/student", web::post().to(add_student))
    .route("/student/{id}", web::get().to(find_student))
    .route("/student/{id}", web::put().to(update_student))
    .route("/student/{id}", web::delete().to(delete_student))

    // application route
    .route("/application", web::get().to(get_applications))
    .route("/application/{id}", web::get().to(get_my_applications))

    // missing route
    .default_service(web::route().to(fallback));
}