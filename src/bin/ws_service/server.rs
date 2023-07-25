use actix::*;
use sqlx::{Pool, Postgres};
use std::{env, time::Duration};
use std::{
  io::Error,
  net::TcpListener
};
use rate_limit::{
  RateLimiter,
  MemoryStore,
  MemoryStoreActor
};
use actix_web::{
  App,
  web::Data,
  HttpServer,
  dev::Server,
  http::header,
  middleware::{Logger, DefaultHeaders}
};

use crate::{
  routes::service_config,
  structs::{
    main_struct::WSAppState,
    message_struct::ChatServer
  }
};

use scholarmate_api::config::{init_cors, init_json_config};

#[doc = "Initialize Actix Web Server"]
pub fn run(listener: TcpListener, database: Pool<Postgres>, app_name_slug: String, app_version: String, max_request: usize, reset_request: u64) -> anyhow::Result<Server, Error> {
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "actix_web=info");
  }

  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let server = ChatServer::new().start();
  let rate_limitter = MemoryStore::new();

  let server: Server = HttpServer::new(move || {
    let allowed_headers = vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE, header::ORIGIN];
    let cors = init_cors(allowed_headers);
    let json_config = init_json_config(104857600);

    App::new()
      .wrap(cors)
      .wrap(Logger::default())
      .wrap(DefaultHeaders::new().add((app_name_slug.as_str(), app_version.as_str())))
      .wrap(RateLimiter::new(MemoryStoreActor::from(rate_limitter.clone()).start()).with_interval(Duration::from_millis(reset_request)).with_max_requests(max_request))
      .app_data(json_config)
      .app_data(Data::new(WSAppState { db: database.clone(), srv: server.clone() }))
      .configure(service_config)
  })
  .listen(listener)?
  .run();

  Ok(server)
}