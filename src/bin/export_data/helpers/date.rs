use chrono::Local;

#[doc = "Get current time"]
pub fn get_current_time() -> String {
  let current_datetime = Local::now();
  let formatted_datetime = current_datetime.format("%y%m%d%H%M%S").to_string();

  formatted_datetime
}