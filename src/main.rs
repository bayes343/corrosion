mod lib;
mod pages;
mod components;
mod layouts;
mod styles;
mod scripts;
use crate::lib::{ utils, build::build, };

fn main() {
    utils::clean();

    build(vec![
        (&layouts::default(), pages::index()),
        (&layouts::default(), pages::not_found())
    ]);
}
