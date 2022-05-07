mod lib;
use crate::lib::{ utils, build::build };

fn main() {
    utils::clean();
    build();
}
