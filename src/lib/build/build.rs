use std::fs;
use crate::lib::domain::{ Page };

pub fn build(pages: Vec<Page>) {
  let public_dir = "./public";

  fs::create_dir("./public").unwrap_or_default();

  for page in pages {
      let contents = page.render();

      let file_path = format!("{}/{}.html", public_dir, page.path);
      fs::write(file_path, contents).unwrap_or_default();
  }
}
