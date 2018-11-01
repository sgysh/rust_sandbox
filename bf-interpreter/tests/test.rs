extern crate bf_interpreter;

#[test]
fn hello_world() {
    let mut input = std::io::Cursor::new("++++++++++[>+++++++>+++++++++>++++++++++>+++++++++++>+++>+<<<<<<-]>++.>>+.>--..+++.>++.<<<---.>>.+++.------.<-.>>+.>.");
    let mut output = Vec::new();

    assert!(bf_interpreter::execute(&mut output, &mut input).is_ok());
    assert_eq!(output, b"Hello World!\n");
}

#[test]
fn only_close_parenthesis() {
    let mut input = std::io::Cursor::new("]");
    let mut output = Vec::new();

    assert!(bf_interpreter::execute(&mut output, &mut input).is_err());
}
