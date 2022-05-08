mod lib;
mod pages;
use crate::lib::{ utils, build::build };

fn main() {
    utils::clean();

    build(vec![
        pages::index(),
        pages::not_found()
    ]);
}
