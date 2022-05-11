pub struct Layout {
    pub template: Box<dyn Fn() -> String>
}

const TITLE_TAG: &str = "<title></title>";
const MAIN_TAG: &str = "<main></main>";

impl Layout {
    pub fn render(&self, name: String, rendered_page: String) -> String {
        (self.template)()
            .replace(TITLE_TAG, &format!("<title>{}</title>", name))
            .replace(MAIN_TAG, &format!("<main>{}</main>", rendered_page))
    }
}
