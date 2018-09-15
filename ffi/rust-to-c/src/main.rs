extern crate libc;

extern {
    fn add_one(input: libc::c_int) -> libc::c_int;
    fn add_two(input: libc::c_int) -> libc::c_int;
}

fn main() {
    let input = 2;
    let output = unsafe { add_one(input) };
    println!("{} + 1 = {}", input, output);
    let output = unsafe { add_two(input) };
    println!("{} + 2 = {}", input, output);
}
