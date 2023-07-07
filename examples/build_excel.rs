use chrono::Local;
use dotenv::dotenv;
use xlsxwriter::prelude::*;
use sqlx::{Pool, Postgres};

use scholarmate_api::{
  helpers::database::connect_postgres,
  models::auth_model::FilteredUserModel
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();
  
  // Connect to database
  let pool = connect_postgres().await;

  // Specify the file path to save the Excel file
  let current_datetime = Local::now();
  let formatted_datetime = current_datetime.format("%Y-%m-%d-%H-%M-%S").to_string();
  let file_path = format!("export_results/{}.xlsx", formatted_datetime);

  // Export data to Excel
  let _ = export_data_to_excel(pool, file_path.as_str()).await;

  println!("Data exported to Excel successfully!");

  Ok(())
}
  
async fn export_data_to_excel(pool: Pool<Postgres>, file_path: &str) -> anyhow::Result<()> {
  // Fetch query data as FilteredUserModel with result type Vec<FilteredUserModel>
  let users = sqlx::query_as!(FilteredUserModel, "select id, email, role from users")
    .fetch_all(&pool)
    .await
    .unwrap();

  // Create a new workbook
  let workbook = Workbook::new(file_path)?;

  // Add a worksheet to the workbook
  let mut worksheet = workbook.add_worksheet(None)?;

  // Define a format for the header row
  let mut header_format = Format::new();
  header_format.set_bold();
  header_format.set_bg_color(FormatColor::Custom(0xD3D3D3));

  // Write header row to the worksheet
  worksheet.write_string(0, 0, "ID", Some(&header_format))?;
  worksheet.write_string(0, 1, "Email", Some(&header_format))?;
  worksheet.write_string(0, 2, "Role", Some(&header_format))?;

  // Write data rows to the worksheet
  for (index, user) in users.iter().enumerate() {
    worksheet.write_number(index as u32 + 1, 0, user.id as f64, None)?;
    worksheet.write_string(index as u32 + 1, 1, &user.email, None)?;
    worksheet.write_string(index as u32 + 1, 2, &user.role, None)?;
  }

  // Close the workbook and save it to the file
  workbook.close()?;

  Ok(())
}

