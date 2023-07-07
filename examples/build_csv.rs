use chrono::Local;
use dotenv::dotenv;
use std::error::Error;
use csv::WriterBuilder;
use sqlx::{Pool, Postgres};

use scholarmate_api::{
  helpers::database::connect_postgres,
  models::auth_model::FilteredUserModel,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  
  // Connect to database
  let pool = connect_postgres().await;

  // Specify the file path to save the CSV file
  let current_datetime = Local::now();
  let formatted_datetime = current_datetime.format("%Y-%m-%d-%H-%M-%S").to_string();
  let file_path = format!("export_results/{}.csv", formatted_datetime);

  // Export data to CSV
  let _ = export_data_to_csv(pool, file_path.as_str()).await;

  println!("Data exported to CSV successfully!");

  Ok(())
}

async fn export_data_to_csv(pool: Pool<Postgres>, file_path: &str) -> anyhow::Result<()> {
  // Fetch query data as FilteredUserModel with result type Vec<FilteredUserModel>
  let users = sqlx::query_as!(FilteredUserModel, "select id, email, role from users")
    .fetch_all(&pool)
    .await
    .unwrap();

  // Create a new CSV writer
  let mut writer = WriterBuilder::new()
    .has_headers(true)
    .from_path(file_path)?;

  // Write data rows to the CSV file
  for user in users {
    writer.serialize(user).unwrap();
  }

  // Flush the CSV writer
  writer.flush()?;

  Ok(())
}