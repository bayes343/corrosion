use crate::lib::domain::{ Script };
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet_function() {
  alert("Hello rust!");
}

pub fn greet() -> Script {
  Script {
    functions: vec![
      format!("greet_function")
    ]
  }
}
