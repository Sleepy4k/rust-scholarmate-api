#[doc = "parsing strings into Vec<u32> types"]
pub fn string_to_vec(input: &str) -> Vec<u32> {
  input
    .trim_end_matches(',')
    .split(',')
    .filter_map(|s| s.parse::<u32>().ok())
    .collect()
}