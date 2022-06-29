use crate::lib::domain::{ Script };
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_u32(a: u32);
}

#[wasm_bindgen]
pub fn greet_function() {
  alert("Hello rust!");
  log("Hello rust!");
}

pub fn greet() -> Script {
  Script {
    functions: vec![
      format!("greet_function")
    ]
  }
}
