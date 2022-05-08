use std::fs;
use crate::lib::domain::{ Page };

pub fn pages(pages: Vec<Page>) {
    let template_index = "./src/wwwroot/index.html";
    let public_dir = "./public";
    let root_element = "<root></root>";
    let default_title = "<title>Corrosion</title>";

    fs::create_dir("./public").unwrap_or_default();

    for page in pages {
        if let Ok(mut contents) = fs::read_to_string(template_index) {
            let page_title = format!("<title>{}</title>", &page.name);
            let rendered_elements: String = page.elements.into_iter().map(|x| x.render()).collect();
            let page_content = format!("<main>{}</main>", rendered_elements);

            contents = contents
                .replace(default_title, &page_title)
                .replace(root_element, &page_content);

            let file_path = format!("{}/{}.html", public_dir, page.path);
            fs::write(file_path, contents).unwrap_or_default();
        };
    }
}
