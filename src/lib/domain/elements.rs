pub enum Elements {
  Heading(u8, String),
  Paragraph(String)
}

impl Elements {
  pub fn render(&self) -> String {
    match self {
      Elements::Heading(level, inner_text) =>
        format!("<h{l}>{i}</h{l}>", l = level, i = inner_text),
      Elements::Paragraph(inner_text) =>
        format!("<p>{}</p>", inner_text)
    }
  }
}
