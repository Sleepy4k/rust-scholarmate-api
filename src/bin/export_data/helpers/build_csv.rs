use csv::WriterBuilder;
use serde_json::{Value, Map};
use std::{path::PathBuf, collections::BTreeMap};

use crate::helpers::parse::*;

use scholarmate_api::helpers::parse::{to_str, to_array};

#[doc = "Write csv file"]
async fn write_csv_file(path: PathBuf, data: Vec<Map<String, Value>>, ref_header: BTreeMap<String, String>, sort_header: Vec<String>) -> anyhow::Result<()> {
  let mut workbook = WriterBuilder::new()
    .has_headers(true)
    .delimiter(b',')
    .from_path(path)?;
  
  for (row_idx, row) in data.into_iter().enumerate() {
    if !sort_header.is_empty() {
      if row_idx == 0 {
        let translated_header = sort_header.iter().map(|x| {
          ref_header.get(x).unwrap_or(x).to_owned()
        }).collect::<Vec<String>>();

        workbook.write_record(&translated_header)?;
      }

      let record_data = sort_header.iter().map(|x| {
        row.get(x).unwrap_or(&Value::Null).to_owned().to_string()
      }).collect::<Vec<String>>();
      
      workbook.write_record(&record_data)?;
    } else {
      if row_idx == 0 {
        let translated_header = row.keys().cloned().map(|x| {
          ref_header.get(&x).unwrap_or(&x).to_owned()
        }).collect::<Vec<String>>();

        workbook.write_record(&translated_header)?;
      }

      let record_data = row.into_iter().map(|(_y, x)| {
        x.to_string()
      }).collect::<Vec<String>>();

      workbook.write_record(&record_data)?;
    }
  };

  workbook.flush()?;

  Ok(())
}

#[doc = "Build csv file"]
#[allow(clippy::redundant_pattern_matching)]
pub async fn build_csv_file(param: (Vec<Map<String, Value>>, Vec<Value>), fields: Value, path: PathBuf) -> anyhow::Result<Vec<u8>> {
  let sort_field = to_array(fields)
    .into_iter().map(|x| {
      to_str(x)
    }).collect::<Vec<String>>();

  let (result, data_ref_ui_column) = param;
  let new_ref_ui_column = mapping_translate_data(data_ref_ui_column);

  if let Ok(_) = write_csv_file(path.clone(), result, new_ref_ui_column, sort_field).await {};
  let data = if let Ok(buff) = get_file_path(path) { buff } else { vec![] };

  Ok(data)
}