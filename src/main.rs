use std::env;
use dotenv::dotenv;
use actix_cors::Cors;
use actix_web::{error, http::header, web::JsonConfig, App, HttpResponse, HttpServer, middleware::{Logger, DefaultHeaders}};

use actix_api::*;

extern crate argon2;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();

  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "actix_api=debug,actix_web=info");
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

  let _database = connect_postgres().await;

  let server = HttpServer::new(|| {
    let cors = Cors::default()
      .allow_any_origin()
      .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
      .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
      .allowed_header(header::CONTENT_TYPE)
      .supports_credentials()
      .max_age(3600);

    let json_config = JsonConfig::default()
      .limit(104857600)
      .error_handler(|err, _req| {
        error::InternalError::from_response(
          err,
          HttpResponse::Conflict().finish(),
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
      .configure(routes::config)
  })
  .bind(server_url.to_owned())?
  .run();
  
  println!("Server running at http://{}/", server_url);

  server.await?;

  Ok(())
}
