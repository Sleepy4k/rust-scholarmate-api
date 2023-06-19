use std::{env, process};
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::postgres::{PgPool, PgPoolOptions};

lazy_static! {
  pub static ref POOL: AsyncOnce<PgPool> = AsyncOnce::new(async  {
    let client = open_postgres().await;

    client
  });
}

#[doc = "Open connection to postgres database"]
pub async fn open_postgres() -> PgPool {
  let database_url = env::var("DATABASE_URL")
    .unwrap_or_else(|_| String::from("postgres://postgres:postgres@localhost:5137/postgres"));
  
  println!("database connect to {}", database_url);

  let pool = match PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await {
      Ok(pool) => {
        println!("Connection to the database is successful!");
        pool
      },
      Err(err) => {
        println!("Failed to connect to the database: {:?}", err);
        process::exit(1);
      }
    };

  pool
}

#[doc = "Connect to postgres database"]
pub async fn connect_postgres() -> PgPool {
  let db = POOL.get().await.to_owned();

  db
}