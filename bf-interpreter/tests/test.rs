extern crate bf_interpreter;

#[test]
fn execute() {
    let mut input = std::io::Cursor::new("++++++++++[>+++++++>+++++++++>++++++++++>+++++++++++>+++>+<<<<<<-]>++.>>+.>--..+++.>++.<<<---.>>.+++.------.<-.>>+.>.");
    let mut output = Vec::new();

    bf_interpreter::execute(&mut output, &mut input);
    assert_eq!(output, b"Hello World!\n");
}
