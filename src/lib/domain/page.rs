use crate::lib::domain::{ Elements };

pub struct Page {
  pub path: String,
  pub name: String,
  pub elements: Vec<Elements>
}

impl Page {
	pub fn render(&self) -> String {
		let rendered_elements: String = (&self.elements).into_iter().map(|e| e.render()).collect();

		format!("<!DOCTYPE html>
	<html lang=\"en\">
	<head>
	<meta charset=\"UTF-8\">
	<meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
	<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
	<title>{}</title>
	</head>
	<body>
	<main>{}</main>
	</body>
	</html>",
		self.name,
		rendered_elements)
	}
}
