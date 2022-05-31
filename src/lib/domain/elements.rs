use crate::lib::domain::{ Component };

pub enum Content {
  InnerText(String),
  InnerHtml(Vec<Elements>)
}

pub enum Elements {
  Text(String),
  Custom(String, Content, Option<Vec<(String, String)>>),
  Heading(u8, Content),
  Paragraph(Content),
  Anchor(String, Content),
  Component(Component)
}

impl Elements {
  pub fn render(&self) -> String {
    match self {
      Elements::Text(text) => text.to_string(),
      Elements::Custom(tag, content, attributes) =>
        raw_html(tag.to_string(), &content, attributes),
      Elements::Heading(level, content) => {
        let tag = format!("h{}", level);
        raw_html(tag, content, &None)
      },
      Elements::Paragraph(content) =>
        raw_html("p".to_string(), content, &None),
      Elements::Anchor(href, content) =>
        raw_html("a".to_string(), content, &Some(vec![
          (format!("href"), href.to_string())
        ])),
      Elements::Component(component) => {
        let els = &component.elements;
        els.into_iter().map(|e| e.render()).collect()
      }
    }
  }
}

fn raw_html(tag: String, content: &Content, attributes: &Option<Vec<(String, String)>>) -> String {
  let mut attributes_string = format!("");
  if let Some(vec) = attributes {
    attributes_string = get_attribute_string(vec)
  }

  format!("<{t}{a}>{i}</{t}>",
    t = tag,
    i = get_inner_html(content),
    a = attributes_string
  )
}

fn get_attribute_string(attributes: &Vec<(String, String)>) -> String {
  attributes.into_iter().map(|a| format!(" {}=\"{}\"", a.0, a.1)).collect()
}

fn get_inner_html(content: &Content) -> String {
  let mut inner_html = String::from("");

  if let Content::InnerText(text) = content {
    inner_html = text.to_string();
  } else if let Content::InnerHtml(elements) = content {
    inner_html = elements.into_iter().map(|e| e.render()).collect();
  }

  inner_html
}
