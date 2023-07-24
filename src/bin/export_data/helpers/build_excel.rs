use serde_json::{Value, Map};
use xlsxwriter::{Workbook, Worksheet};
use std::{path::PathBuf, collections::BTreeMap};

use crate::helpers::parse::*;

use scholarmate_api::helpers::parse::{to_str, to_f64, to_array};

#[doc = "Write cell to excel file"]
fn write_excel_cell(row: u32, col: u16, value: Value, sheet: &mut Worksheet) -> anyhow::Result<()> {
  if value.is_i64() || value.is_f64() {
    sheet.write_number(row, col, to_f64(value), None)?
  } else if value.is_string() {
    sheet.write_string(row, col, to_str(value).as_str(), None)?
  } else {
    sheet.write_blank(row, col, None)?
  };

  Ok(())
}

#[doc = "Write excel file"]
async fn write_excel_file(path: PathBuf, data: Vec<Map<String, Value>>, ref_header: BTreeMap<String, String>, sort_header: Vec<String>) -> anyhow::Result<()> {
  let workbook = Workbook::new(path
    .as_path()
    .as_os_str()
    .to_str()
    .unwrap_or_default())?;
  
  let mut sheet = workbook.add_worksheet(None)?;

  for (row_idx, row) in data.into_iter().enumerate() {
    if !sort_header.is_empty() {
      if row_idx == 0 {
        for (col_idx, x) in sort_header.iter().enumerate() {
          let display_header = ref_header.get(x).unwrap_or(x).to_owned();
          sheet.write_string(row_idx as u32, col_idx as u16, &display_header, None)?;
        }
      }

      for (col_idx, x) in sort_header.iter().enumerate() {
        let values = row.get(x).unwrap_or(&Value::Null).to_owned();
        write_excel_cell((row_idx + 1) as u32, col_idx as u16, values, &mut sheet)?;                  
      }
    } else {
      if row_idx == 0 {
        let keys = row.keys().cloned().collect::<Vec<String>>();

        for (col_idx, x) in keys.into_iter().enumerate() {
          let display_header = ref_header.get(&x).unwrap_or(&x).to_owned();
          sheet.write_string(row_idx as u32, col_idx as u16, &display_header, None)?;
        };
      }

      for (col_idx, (_y, x)) in row.into_iter().enumerate() {
        write_excel_cell((row_idx + 1) as u32, col_idx as u16, x, &mut sheet)?;             
      };
    }
  };

  workbook.close()?;

  Ok(())
}

#[doc = "Build excel file"]
#[allow(clippy::redundant_pattern_matching)]
pub async fn build_excel_file(param: (Vec<Map<String, Value>>, Vec<Value>), fields: Value,path: PathBuf) -> anyhow::Result<Vec<u8>> {
  let sort_field = to_array(fields)
    .into_iter().map(|x| {
      to_str(x)
    }).collect::<Vec<String>>();

  let (result, data_ref_ui_column) = param;
  let new_ref_ui_column = mapping_translate_data(data_ref_ui_column);

  if let Ok(_) = write_excel_file(path.clone(), result, new_ref_ui_column, sort_field).await {};
  let data = if let Ok(buff) = get_file_path(path) { buff } else { vec![] };

  Ok(data)
}
