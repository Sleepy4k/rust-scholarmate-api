use serde_json::{Value, Map};
use std::{path::PathBuf, fs::File, io::Read, collections::BTreeMap};

use scholarmate_api::helpers::parse::{to_str, map_get};

#[doc = "Build data from json to Vec<Map<String, Value>>"]
#[allow(clippy::map_clone)]
pub async fn build_data(data: Vec<Value>) -> anyhow::Result<Vec<Map<String, Value>>> {
  let result = data
    .into_iter()
    .filter_map(|value| value.as_object()
    .map(|map| map.clone()))
    .collect();

  Ok(result)
}

#[doc = "Get translate data from translate_columns to BTreeMap<String, String>"]
pub fn mapping_translate_data(refs: Vec<Value>) -> BTreeMap<String, String> {
  refs.into_iter().map(|x| {
    (
      to_str(map_get("column_name", x.clone())), 
      to_str(map_get("language", x.clone()))
    )
  }).collect::<BTreeMap<String, String>>()
}

#[doc = "Get file path and return Vec<u8>"]
pub fn get_file_path(path: PathBuf) -> anyhow::Result<Vec<u8>> {
  let mut file = File::open(path.clone())?;
  let mut res = vec![];
  file.read_to_end(&mut res)?;

  match std::fs::remove_file(path.clone()) {
    Ok(()) => println!("Remove file: {}!", path.file_name().unwrap_or_default().to_str().unwrap_or_default()),
    Err(e) => println!("Failed to remove file!, \n{}", e)
  };

  Ok(res)
}
