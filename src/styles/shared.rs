use crate::lib::domain::{ Attributes, Style };

pub fn shared() -> Style {
  Style {
    target: format!("body"),
    attributes: vec![
      Attributes::Custom(format!("display"), format!("flex")),
      Attributes::Custom(format!("flex-direction"), format!("column")),
      Attributes::Custom(format!("color"), format!("#808080"))
    ]
  }
}
