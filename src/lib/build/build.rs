use std::fs;
use crate::lib::domain::{ Page, Layout };

pub fn build(pages: Vec<(&Layout, Page)>) {
    let public_dir = "./public";

    fs::create_dir("./public").unwrap_or_default();

    for page in pages {
        let page_render = page.1.render();
        let layout_render = page.0.render(page.1.name, page_render);
        let file_path = format!("{}/{}.html", public_dir, page.1.path);
        fs::write(file_path, layout_render).unwrap_or_default();
    }
}
