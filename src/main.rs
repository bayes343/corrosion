mod lib;
mod pages;
mod components;
mod layouts;
use crate::lib::{ utils, build::build, };

fn main() {
    utils::clean();

    build(vec![
        (&layouts::default(), pages::index()),
        (&layouts::default(), pages::not_found())
    ]);
}
