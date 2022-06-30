use crate::lib::domain::{ Component, Script };
use uuid::Uuid;

pub enum Content {
  InnerText(String),
  InnerHtml(Vec<Elements>)
}

pub enum Events {
  OnClick(Script)
}

impl Events {
  pub fn render(&self, elementId: String) -> String {
    match self {
      Events::OnClick(script) => script.render(Some((elementId, format!("click"))))
    }
  }
}

pub enum Elements {
  Text(String),
  Custom(String, Content, Option<Vec<(String, String)>>),
  Heading(u8, Content),
  Paragraph(Content),
  Anchor(String, Content),
  Component(Component),
  Button(Content, Events)
}

impl Elements {
  pub fn render(&self) -> String {
    match self {
      Elements::Text(text) => text.to_string(),
      Elements::Custom(tag, content, attributes) =>
        raw_html(tag.to_string(), &content, attributes, None),
      Elements::Heading(level, content) => {
        let tag = format!("h{}", level);
        raw_html(tag, content, &None, None)
      },
      Elements::Paragraph(content) =>
        raw_html("p".to_string(), content, &None, None),
      Elements::Anchor(href, content) =>
        raw_html("a".to_string(), content, &Some(vec![
          (format!("href"), href.to_string())
        ]), None),
      Elements::Component(component) => {
        let els = &component.elements;
        els.into_iter().map(|e| e.render()).collect()
      },
      Elements::Button(content, event) =>
        raw_html("button".to_string(), content, &None, Some(event))
    }
  }
}

fn raw_html(tag: String, content: &Content, attributes: &Option<Vec<(String, String)>>, event: Option<&Events>) -> String {
  let mut attributes_string = format!("");
  if let Some(vec) = attributes {
    attributes_string = get_attribute_string(vec)
  }
  let inner_html = get_inner_html(content);

  let id = Uuid::new_v4();
  let mut id_attribute = format!("");
  let mut event_script: String = format!("");
  if let Some(event) = event {
    id_attribute = format!(" id=\"{id}\"");
    event_script = event.render(id.to_string());
  }

  format!("<{tag}{id_attribute}{attributes_string}>{inner_html}{event_script}</{tag}>")
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
