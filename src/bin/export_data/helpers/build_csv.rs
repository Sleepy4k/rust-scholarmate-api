use serde_json::Value;

use scholarmate_api::helpers::parse::{to_str, map_get};

pub async fn write_csv_file(data: Vec<Vec<(String, Value)>>) -> String {
  let mut data_str = String::new();

  for (row_idx, row) in data.into_iter().enumerate() {
    if row_idx == 0 {
      let keys = row.clone().into_iter().map(|x| x.0).collect::<Vec<_>>();

      data_str.push_str(keys.join("|").as_str());
      data_str.push('\n');
    }

    let values = row.clone().into_iter().map(|x| x.1.to_string()).collect::<Vec<_>>();

    data_str.push_str(values.join("|").as_str());
    data_str.push('\n');
  }

  data_str
}

pub fn build_csv_file(data: Value, headers: Vec<Value>) -> Vec<(String, Value)> {
  let mut formatted_data = Vec::new();

  for header in headers {
    formatted_data.push((header["language"].to_string(), map_get(&to_str(header["column_name"].clone()), data.to_owned())));
  }

  formatted_data
}