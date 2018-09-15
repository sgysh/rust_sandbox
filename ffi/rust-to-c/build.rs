extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/add_one.c")
        .file("src/add_two.c")
        .compile("libaddone.a");
}
