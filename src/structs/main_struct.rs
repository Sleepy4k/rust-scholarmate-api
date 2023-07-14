use sqlx::{Pool, Postgres};

#[doc = "App state struct"]
#[derive(Debug)]
pub struct AppState {
  pub db: Pool<Postgres>,
}
  