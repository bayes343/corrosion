use crate::lib::domain::{ Page, Elements };

pub fn index() -> Page {
  Page {
    path: String::from("index"),
    name: String::from("Home"),
    elements: vec![
      Elements::Heading(1, String::from("Home")),
      Elements::Paragraph(String::from("Experimental frontend framework using the rust language."))
    ]
  }
}
