use crate::lib::domain::{ Component };

pub enum Elements {
  Heading(u8, Content),
  Paragraph(Content),
  Component(Component),
  Custom(String, Content),
  Text(String)
}

pub enum Content {
  InnerText(String),
  InnerHtml(Vec<Elements>)
}

impl Elements {
  pub fn render(&self) -> String {
    match self {
      Elements::Heading(level, content) =>
        format!("<h{l}>{i}</h{l}>", l = level, i = get_inner_html(content)),
      Elements::Paragraph(content) =>
        format!("<p>{}</p>", get_inner_html(content)),
      Elements::Component(component) => {
        let els = &component.elements;
        els.into_iter().map(|e| e.render()).collect()
      },
      Elements::Custom(tag, content) =>
        format!("<{t}>{i}</{t}>", t = tag, i = get_inner_html(content)),
      Elements::Text(text) => text.to_string()
    }
  }
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
