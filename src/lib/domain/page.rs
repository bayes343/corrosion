use crate::lib::domain::{ Elements, Style };

pub struct Page {
  pub path: String,
  pub name: String,
  pub elements: Vec<Elements>,
  pub styles: Vec<Style>
}

impl Page {
	pub fn render(&self) -> String {
    let rendered_styles: String = (&self.styles).into_iter().map(|s| s.render()).collect();
    let rendered_elements: String = (&self.elements).into_iter().map(|e| e.render()).collect();

    format!(
      "<style>{}</style>{}",
      rendered_styles,
      rendered_elements
    )
	}
}
