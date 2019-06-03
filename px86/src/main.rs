fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        std::process::exit(1);
    }

    let mut input = std::fs::File::open(&args[1]).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    if let Err(e) = px86::execute(&mut input) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
