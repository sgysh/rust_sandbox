extern crate bf_interpreter;

use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input = File::open(&args[1]).unwrap();

    let output = std::io::stdout();
    let mut output = output.lock();

    bf_interpreter::execute(&mut output, &mut input);
}
