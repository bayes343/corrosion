use crate::lib::domain::{ Component, Elements, Content };

pub fn nav() -> Component {
  Component {
    elements: vec![
      Elements::Custom(
        format!("header"), Content::InnerHtml(
        vec![
          Elements::Anchor("index.html".to_string(), Content::InnerText("Home".to_string())),
          Elements::Text(format!(" | ")),
          Elements::Anchor("404.html".to_string(), Content::InnerText("Not Found".to_string()))
        ]
      ),
      None)
    ]
  }
}
