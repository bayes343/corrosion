pub struct Script {
  pub functions: Vec<String>
}

impl Script {
  pub fn render(&self) -> String {
    let mut script_module = format!("");

    let functions = &self.functions;
    let function_names: String = functions.into_iter().map(|s| format!("{},", s)).collect();
    let function_invocations: String = functions.into_iter().map(|s| format!("{}();", s)).collect();

    if function_names.len() > 0 {
      script_module = format!("<script type=\"module\">
      import init, {{ {function_names} }} from '../pkg/corrosion.js';
      async function main() {{
          await init();
          {function_invocations}
      }}
      main();
    </script>");
    }

    script_module
  }
}
