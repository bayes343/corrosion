pub struct Script {
  pub functions: Vec<String>
}

impl Script {
  pub fn render(&self, (elementIdEventName): Option<(String, String)>) -> String {
    let mut script_module = format!("");

    let functions = &self.functions;
    let function_names: String = functions.into_iter().map(|s| format!("{},", s)).collect();
    let mut function_invocations: String = functions.into_iter().map(|s| format!("{}();", s)).collect();

    if let Some(id_event) = elementIdEventName {
      let id = id_event.0;
      let event = id_event.1;
      function_invocations = format!("document.getElementById('{id}')?.addEventListener('{event}', () => {{{function_invocations}}})")
    }

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
