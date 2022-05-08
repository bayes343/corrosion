use crate::lib::domain::{ Component, Elements, Content };

pub fn header(heading: String, sub_heading: String) -> Component {
  Component {
    elements: vec![
      Elements::Heading(1, Content::InnerText(heading)),
      Elements::Paragraph(Content::InnerText(sub_heading))
    ]
  }
}
