use crate::lib::domain::{ Page, Elements };
use crate::components::{ header };

pub fn not_found() -> Page {
  Page {
    path: String::from("404"),
    name: String::from("Not Found"),
    elements: vec![
      Elements::Component(header(
        String::from("Page Not Found"),
        String::from("Please check your spelling or try back later.")
      ))
    ]
  }
}
