use crate::lib::domain::{ Elements, Style, Script };

pub struct Page {
  pub path: String,
  pub name: String,
  pub elements: Vec<Elements>,
  pub styles: Vec<Style>,
  pub script: Option<Script>
}

impl Page {
	pub fn render(&self) -> String {
    let rendered_styles: String = (&self.styles).into_iter().map(|s| s.render()).collect();
    let rendered_elements: String = (&self.elements).into_iter().map(|e| e.render()).collect();
    let mut rendered_script: String = format!("");

    if let Some(script) = &self.script {
      rendered_script = script.render();
    }

    format!(
      "<style>{rendered_styles}</style>
      {rendered_elements}
      {rendered_script}"
    )
	}
}
