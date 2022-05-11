use crate::lib::domain::{ Component, Elements, Content };

pub fn nav() -> Component {
  Component {
    elements: vec![
      Elements::Custom(format!("header"), Content::InnerHtml(
        vec![
          Elements::Paragraph(Content::InnerText(format!("Nav goes here")))
        ]
      ))
    ]
  }
}
