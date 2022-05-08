use crate::lib::domain::{ Page, Elements };

pub fn not_found() -> Page {
  Page {
    path: String::from("404"),
    name: String::from("Not Found"),
    elements: vec![
      Elements::Heading(1, String::from("Page Not Found")),
      Elements::Paragraph(String::from("Please check your spelling or try back later."))
    ]
  }
}
