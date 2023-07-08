use std::env;
use dotenv::dotenv;
use actix_cors::Cors;
use serde_json::json;
use actix_web::{
  App,
  error,
  HttpServer,
  HttpResponse,
  http::header,
  web::{Data, JsonConfig},
  middleware::{Logger, DefaultHeaders}
};

use scholarmate_api::{
  structs::main_struct::AppState,
  helpers::{
    parse::slugify,
    database::connect_postgres
  }
};

mod routes;
mod schemas;
mod helpers;
mod controllers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();

  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "actix_web=info");
  }

  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let port = env::var("EXPORT_DATA_PORT")
    .expect("no environment variable set for \"EXPORT_DATA_PORT\"")
    .parse::<u16>()
    .unwrap_or(8081);

  let hostname = env::var("EXPORT_DATA_HOST")
    .expect("no environment variable set for \"EXPORT_DATA_HOST\"")
    .parse::<String>()
    .unwrap_or_else(|_| String::from("localhost"));

  let server_url = format!("{}:{}", hostname, port);

  println!("Server running at http://{}/", server_url.to_owned());

  let database = connect_postgres().await;

  let _ = HttpServer::new(move || {
    let cors = Cors::default()
      .allow_any_origin()
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
          "message": message,
          "data": null
        });

        error::InternalError::from_response(
          err,
          HttpResponse::InternalServerError().json(response),
        )
        .into()
      });

    let app_name = env::var("EXPORT_DATA_NAME")
    .expect("EXPORT_DATA_NAME not set")
    .parse::<String>()
    .unwrap_or_else(|_| String::from("Actix API"));

    let app_name_slug = slugify(&format!("{}-{}", app_name, "version"));

    let app_version = env::var("EXPORT_DATA_VERSION")
    .expect("EXPORT_DATA_VERSION not set")
    .parse::<String>()
    .unwrap_or_else(|_| String::from("1.0.0"));

    App::new()
    .wrap(cors)
    .wrap(Logger::default())
    .wrap(DefaultHeaders::new().add((app_name_slug.as_str(), app_version.as_str())))
    // .wrap(cookie::CheckCookie)
    .app_data(json_config)
    .app_data(Data::new(AppState { db: database.clone() }))
    .configure(routes::config)
  })
  .bind(server_url)?
  .run()
  .await?;

  Ok(())
}