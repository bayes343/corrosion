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

    let mut function_names = format!("");
    let mut function_invocations = format!("");
    let mut script_module = format!("");

    if let Some(script) = &self.script {
      let functions = &script.functions;
      function_names = functions.into_iter().map(|s| format!("{},", s)).collect();
      function_invocations = functions.into_iter().map(|s| format!("{}();", s)).collect();

      if function_names.len() > 0 {
        script_module = format!("<script type=\"module\">
        import init, {{ {function_names} }} from '../pkg/corrosion.js';
        async function main() {{
            await init();
            {function_invocations}
        }}
        main();
      </script>")
      }
    }

    format!(
      "<style>{rendered_styles}</style>
      {rendered_elements}
      {script_module}"
    )
	}
}
