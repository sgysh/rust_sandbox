extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/add_one.cc")
        .file("src/add_two.cc")
        .cpp(true)
        .compile("libadd.a");
}
