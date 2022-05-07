use std::fs;

pub fn clean() {
  fs::remove_dir_all("./public").unwrap_or_default();
}
