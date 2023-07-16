use std::env;
use actix_cors::Cors;
use serde_json::{json, Value};
use actix_web::{
  error,
  HttpResponse,
  web::JsonConfig,
  http::header::HeaderName,
};

use crate::helpers::parse::slugify;

#[doc = "Initialize Actix Web CORS"]
pub fn init_cors(allowed_headers: Vec<HeaderName>) -> Cors {
  Cors::default()
    .allow_any_origin()
    .allow_any_method()
    .supports_credentials()
    .allowed_headers(allowed_headers)
    .max_age(604800)
}

#[doc = "Initialize Actix Web JSON Config"]
pub fn init_json_config(limit: usize) -> JsonConfig {
  JsonConfig::default()
    .limit(limit)
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
    })
}

#[doc = "Get Environment Variable"]
fn get_env_var(server_config: &Value, key: &str) -> Option<String> {
  server_config[key].as_str().map(String::from)
}

#[doc = "Get Server Config"]
fn get_server_config(name: &str, version: &str, host: &str, port: &str, max_request: &str, reset_request: &str,) -> Value {
  let server_type = json!({
    "name": env::var(name).expect(&format!("no environment variable set for \"{}\"", name)),
    "version": env::var(version).expect(&format!("no environment variable set for \"{}\"", version)),
    "host": env::var(host).expect(&format!("no environment variable set for \"{}\"", host)),
    "port": env::var(port).expect(&format!("no environment variable set for \"{}\"", port)),
    "max_request": env::var(max_request).expect(&format!("no environment variable set for \"{}\"", max_request)),
    "reset_request": env::var(reset_request).expect(&format!("no environment variable set for \"{}\"", reset_request)),
  });

  server_type
}

#[doc = "Initialize Actix Web Server Data"]
pub fn init_server_data(server: &str) -> (String, String, String, usize, u64) {
  let server_config = match server {
    "main" => get_server_config(
      "APP_NAME",
      "APP_VERSION",
      "APP_HOST",
      "APP_PORT",
      "APP_RATE_LIMIT_MAX_REQUEST",
      "APP_RATE_LIMIT_WINDOW_MS",
    ),
    "export_data" => get_server_config(
      "EXPORT_DATA_NAME",
      "EXPORT_DATA_VERSION",
      "EXPORT_DATA_HOST",
      "EXPORT_DATA_PORT",
      "EXPORT_DATA_RATE_LIMIT_MAX_REQUEST",
      "EXPORT_DATA_RATE_LIMIT_WINDOW_MS",
    ),
    "auth_service" => get_server_config(
      "AUTH_SERVICE_NAME",
      "AUTH_SERVICE_VERSION",
      "AUTH_SERVICE_HOST",
      "AUTH_SERVICE_PORT",
      "AUTH_SERVICE_RATE_LIMIT_MAX_REQUEST",
      "AUTH_SERVICE_RATE_LIMIT_WINDOW_MS",
    ),
    "ws_service" => get_server_config(
      "WS_SERVICE_NAME",
      "WS_SERVICE_VERSION",
      "WS_SERVICE_HOST",
      "WS_SERVICE_PORT",
      "WS_SERVICE_RATE_LIMIT_MAX_REQUEST",
      "WS_SERVICE_RATE_LIMIT_WINDOW_MS",
    ),
    _ => panic!("Server type is required"),
  };

  let app_name = get_env_var(&server_config, "name").unwrap_or_else(|| String::from("Actix API"));
  let app_name_slug = slugify(&format!("{}-{}", app_name, "version"), "-");
  let app_version = get_env_var(&server_config, "version").unwrap_or_else(|| String::from("1.0.0"));
  let app_host = get_env_var(&server_config, "host").unwrap_or_else(|| String::from("localhost"));
  let app_port = get_env_var(&server_config, "port").unwrap_or_else(|| String::from("8080"));
  let app_url = format!("{}:{}", app_host, app_port);
  let rate_limit_max_request = get_env_var(&server_config, "max_request").unwrap_or_else(|| String::from("100"));
  let rate_limit_window_ms = get_env_var(&server_config, "reset_request").unwrap_or_else(|| String::from("60000"));

  (
    app_name_slug,
    app_version,
    app_url,
    rate_limit_max_request.parse().unwrap_or(100),
    rate_limit_window_ms.parse().unwrap_or(60000),
  )
}