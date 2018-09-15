#![crate_type = "staticlib"]

#[no_mangle]
pub extern fn add_one(input: i32) -> i32 {
    input + 1
}
