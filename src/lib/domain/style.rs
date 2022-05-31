pub enum Attributes {
  Custom(String, String),
}

impl Attributes {
  pub fn render(&self) -> String {
    match self {
      Attributes::Custom(attribute, value) => format!("{}:{};", attribute, value)
    }
  }
}

pub struct Style {
  pub target: String,
  pub attributes: Vec<Attributes>
}

impl Style {
  pub fn render(&self) -> String {
    let rendered_attributes: String = (&self.attributes)
        .into_iter()
        .map(|a| a.render())
        .collect();

    format!(
      "{} {{{}}}",
      self.target,
      rendered_attributes
    )
  }
}
