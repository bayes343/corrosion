mod lib;
use crate::lib::{ utils, build::build };

fn main() {
    utils::clean();

    let vec = vec![
        String::from("index"),
        String::from("404")
    ];
    build(vec);
}
