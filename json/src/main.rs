extern crate serde_json;

use std::fs::File;
use serde_json::Value;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} FILE", args[0]);
        return 1;
    }

    let file = File::open(&args[1]).unwrap();

    let v: Value = serde_json::from_reader(file).unwrap();

    print_value(v, 0);

    0
}

fn print_value(v: Value, depth: i32) {
    if !v.is_object() {
        println!("{}", v);
        return;
    }

    let obj = v.as_object().unwrap();

    for (name, value) in obj.iter() {
        if value.is_object() {
            print_spaces(depth);
            println!("{} = [", name);

            print_value(value.clone(), depth + 2);

            print_spaces(depth);
            println!("]");
        } else {
            print_spaces(depth);
            println!("{} -> {}", name, value);
        }
    }
}

fn print_spaces(n: i32) {
    for _ in 0..n {
        print!(" ");
    }
}
