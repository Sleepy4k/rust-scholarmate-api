use sqlx::{Pool, Postgres};

#[doc = "App state struct"]
pub struct AppState {
  pub db: Pool<Postgres>,
}
  