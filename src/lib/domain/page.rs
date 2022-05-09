use crate::lib::domain::{ Elements };

pub struct Page {
  pub path: String,
  pub name: String,
  pub elements: Vec<Elements>
}

impl Page {
	pub fn render(&self) -> String {
		(&self.elements).into_iter().map(|e| e.render()).collect()
	}
}
