use crate::lib::domain::{ Component, Elements };

pub fn header(heading: String, sub_heading: String) -> Component {
  Component {
    elements: vec![
      Elements::Heading(1, heading),
      Elements::Paragraph(sub_heading)
    ]
  }
}
