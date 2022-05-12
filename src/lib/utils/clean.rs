use std::fs;
use std::ffi::OsStr;

pub fn clean() {
  for path in fs::read_dir("./public").unwrap() {
    let path = path.unwrap().path();
    let extension = path.extension().unwrap();

    if extension == OsStr::new("html") {
      fs::remove_file(path).unwrap();
    }
  }
}
