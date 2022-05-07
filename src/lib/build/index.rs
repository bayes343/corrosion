use std::fs;

pub fn index() {
    let template_index = "./src/wwwroot/index.html";
    let public_index = "./public/index.html";

    fs::create_dir("./public").unwrap_or_default();

    if let Ok(mut contents) = fs::read_to_string(template_index) {
        contents = contents.replace("<root></root>", "<div></div>");
        fs::write(public_index, contents).unwrap_or_default();
    };
}
