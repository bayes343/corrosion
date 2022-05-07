use std::fs;

pub fn pages(pages: Vec<String>) {
    let template_index = "./src/wwwroot/index.html";
    let public_dir = "./public";

    fs::create_dir("./public").unwrap_or_default();

    for page in pages {
        if let Ok(mut contents) = fs::read_to_string(template_index) {
            contents = contents.replace("<root></root>", "<div></div>");
            let file_path = format!("{}/{}.html", public_dir, page);
            fs::write(file_path, contents).unwrap_or_default();
        };
    }
}
