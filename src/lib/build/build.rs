use crate::lib::build;
use crate::lib::domain::{ Page };

pub fn build(pages: Vec<Page>) {
  build::pages(pages);
}
