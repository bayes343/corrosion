use crate::lib::domain::{ Elements };

pub struct Page {
  pub path: String,
  pub name: String,
  pub elements: Vec<Elements>
}
