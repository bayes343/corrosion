use crate::lib::domain::{ Page, Elements };
use crate::components::{ header };

pub fn index() -> Page {
  Page {
    path: String::from("index"),
    name: String::from("Home"),
    elements: vec![
      Elements::Component(header(
        String::from("Home"),
        String::from("Experimental frontend framework using the rust language.")
      ))
    ]
  }
}
