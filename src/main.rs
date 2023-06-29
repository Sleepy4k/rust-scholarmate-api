use std::env;
use dotenv::dotenv;
use serde_json::json;
use actix_cors::Cors;
use actix_web::{
  App,
  error,
  HttpServer,
  HttpResponse,
  web::{Data, JsonConfig},
  http::header::{self, HeaderValue},
  middleware::{Logger, DefaultHeaders}
};

use scholarmate_api::*;

extern crate argon2;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();

  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "actix_web=info");
  }

  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let port = env::var("APP_PORT")
    .expect("no environment variable set for \"ENV STATUS\"")
    .parse::<u16>()
    .unwrap_or(8080);

  let hostname = env::var("APP_HOST")
    .expect("no environment variable set for \"ENV STATUS\"")
    .parse::<String>()
    .unwrap_or_else(|_| String::from("localhost"));

  let server_url = format!("{}:{}", hostname, port);

  println!("Server running at http://{}/", server_url.to_owned());

  let database = connect_postgres().await;

  let _ = HttpServer::new(move || {
    let cors = Cors::default()
      .allowed_origin_fn(|origin, _| {
        let allowed_origin = env::var("CORS_ORIGIN").unwrap_or("http://localhost:3000".into());
        [
          HeaderValue::from_str(&allowed_origin).unwrap_or(HeaderValue::from_static("http://localhost:3000")),
        ]
        .contains(origin)
      })
      .allowed_headers(vec![
        header::AUTHORIZATION,
        header::ACCEPT,
        header::CONTENT_TYPE,
        header::ORIGIN,
      ])
      .allow_any_method()
      .supports_credentials()
      .max_age(604800);

    let json_config = JsonConfig::default()
      .limit(104857600)
      .error_handler(|err, _req| {
        let message = err.to_string();
        let response = json!({
          "status": "error",
          "message": message
        });

        error::InternalError::from_response(
          err,
          HttpResponse::InternalServerError().json(response),
        )
        .into()
      });

    let app_name = env::var("APP_NAME")
      .expect("APP_NAME not set")
      .parse::<String>()
      .unwrap_or_else(|_| String::from("Actix API"));

    let app_name_slug = slugify(&format!("{}-{}", app_name, "version"));

    let app_version = env::var("APP_VERSION")
      .expect("APP_VERSION not set")
      .parse::<String>()
      .unwrap_or_else(|_| String::from("1.0.0"));

    App::new()
      .wrap(cors)
      .wrap(Logger::default())
      .wrap(DefaultHeaders::new().add((app_name_slug.as_str(), app_version.as_str())))
      .wrap(cookie::CheckCookie)
      .app_data(json_config)
      .app_data(Data::new(main_struct::AppState { db: database.clone() }))
      .configure(routes::config)
  })
  .bind(server_url)?
  .run()
  .await;

  Ok(())
}
