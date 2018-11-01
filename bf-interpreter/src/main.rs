extern crate bf_interpreter;

use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input = File::open(&args[1]).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let output = std::io::stdout();
    let mut output = output.lock();

    if let Err(e) = bf_interpreter::execute(&mut output, &mut input) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
