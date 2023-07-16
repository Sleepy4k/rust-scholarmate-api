extern crate argon2;

use dotenv::dotenv;
use std::net::TcpListener;

use crate::server::run;

use scholarmate_api::{
  config::init_server_data,
  helpers::database::connect_postgres
};

mod server;
mod routes;
mod models;
mod helpers;
mod schemas;
mod controllers;
mod repositories;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();

  let (app_name_slug, app_version, app_url, rate_limit_max_requests, rate_limit_duration) = init_server_data("auth_service");
  let listener = TcpListener::bind(app_url.to_owned())?;
  let database = connect_postgres().await;

  println!("Server running at http://{}/", app_url.to_owned());

  run(listener, database, app_name_slug, app_version, rate_limit_max_requests, rate_limit_duration)?.await?;

  Ok(())
}