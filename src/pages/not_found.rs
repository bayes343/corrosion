use crate::lib::domain::{ Page, Elements, Style, Attributes };
use crate::components::{ header };
use crate::styles::{ shared };
use crate::scripts::{ greet };

pub fn not_found() -> Page {
  Page {
    path: String::from("404"),
    name: String::from("Not Found"),
    elements: vec![
      Elements::Component(header(
        String::from("Page Not Found"),
        String::from("Please check your spelling or try back later.")
      ))
    ],
    styles: vec![
      shared(),
      Style {
        target: format!("h1"),
        attributes: vec![
          Attributes::Custom(format!("color"), format!("red"))
        ]
      }
    ],
    script: Some(greet())
  }
}
