extern crate md5;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("examples/hello_world.txt").unwrap();
    let mut buf = Vec::new();

    file.read_to_end(&mut buf).unwrap();
    println!("{:>032x}", md5::compute(buf));
}
