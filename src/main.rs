use std::env;
use dotenv::dotenv;
use serde_json::json;
use actix_cors::Cors;
use std::time::Duration;
use rate_limit::{
  RateLimiter,
  MemoryStore,
  MemoryStoreActor
};
use actix_web::{
  App,
  error,
  HttpServer,
  HttpResponse,
  http::header,
  web::{Data, JsonConfig},
  middleware::{Logger, DefaultHeaders}
};

use scholarmate_api::*;

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
  let rate_limitter = MemoryStore::new();

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

    let rate_limit_max_requests = env::var("RATE_LIMIT_MAX_REQUEST")
      .expect("RATE_LIMIT_MAX_REQUEST not set")
      .parse::<usize>()
      .unwrap_or(100);

    let rate_limit_duration = env::var("RATE_LIMIT_WINDOW_MS")
      .expect("RATE_LIMIT_WINDOW_MS not set")
      .parse::<u64>()
      .unwrap_or(60000);

    App::new()
      .wrap(cors)
      .wrap(Logger::default())
      .wrap(DefaultHeaders::new().add((app_name_slug.as_str(), app_version.as_str())))
      // .wrap(cookie::CheckCookie)
      .wrap(RateLimiter::new(MemoryStoreActor::from(rate_limitter.clone()).start()).with_interval(Duration::from_millis(rate_limit_duration)).with_max_requests(rate_limit_max_requests))
      .app_data(json_config)
      .app_data(Data::new(main_struct::AppState { db: database.clone() }))
      .configure(routes::config)
  })
  .bind(server_url)?
  .run()
  .await;

  Ok(())
}
