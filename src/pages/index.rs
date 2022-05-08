use crate::lib::domain::{ Page, Elements, Content };
use crate::components::{ header };

pub fn index() -> Page {
  Page {
    path: String::from("index"),
    name: String::from("Home"),
    elements: vec![
      Elements::Component(header(
        String::from("Home"),
        String::from("Experimental frontend framework using the rust language.")
      )),
      Elements::Custom(
        String::from("div"),
        Content::InnerHtml(vec![
          Elements::Paragraph(
            Content::InnerHtml(vec![
              Elements::Text(String::from("This text is ")),
              Elements::Custom(String::from("b"), Content::InnerText(String::from("bold")))
            ]))
        ])
      )
    ]
  }
}
