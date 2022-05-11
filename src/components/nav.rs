use crate::lib::domain::{ Component, Elements, Content };

pub fn nav() -> Component {
  Component {
    elements: vec![
      Elements::Custom(
        format!("header"), Content::InnerHtml(
        vec![
          Elements::Custom(format!("a"), Content::InnerText(format!("Home")), Some(vec![
            (format!("href"), format!(""))
          ]))
        ]
      ),
      None)
    ]
  }
}
