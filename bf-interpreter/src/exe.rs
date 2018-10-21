use mem;
use std::io;
use std::io::{Read, Write, Seek, SeekFrom};

fn skip_to_closing_parenthesis<R: Read>(r: &mut R, open: char, close: char) {
    let mut nest = 1;
    let mut b = [0_u8; 1];

    while nest != 0 {
        r.read(&mut b).unwrap();
        let c = b[0];

        if c == open as u8 {
            nest += 1;
        } else if c == close as u8 {
            nest -= 1;
        }
    }
}

/// ## Example
/// ```
/// let mut input = std::io::Cursor::new("++++++++[>++++++++<-]>+.+.+.");
/// let mut output = Vec::new();
///
/// bf_interpreter::execute(&mut output, &mut input);
/// assert_eq!(output, b"ABC");
/// ```
pub fn execute<O: Write, I: Read + Seek>(output: &mut O, input: &mut I) {
    let mut mem: mem::Memory = mem::Memory::init(30000);
    let mut stack = Vec::new();
    let mut b = [0_u8; 1];

    while input.read(&mut b).unwrap() != 0 {
        let c = b[0] as char;
        match c {
            '+' => mem.add(1),
            '-' => mem.sub(1),
            '>' => mem.next(),
            '<' => mem.prev(),
            '[' => {
                if mem.val() != 0 {
                    stack.push(input.seek(SeekFrom::Current(0)));
                } else {
                    skip_to_closing_parenthesis(input, '[', ']');
                }
            },
            ']' => {input.seek(SeekFrom::Start(stack.pop().unwrap().unwrap() - 1)).unwrap();},
            '.' => {write!(output, "{}", mem.val() as char).unwrap();},
            ',' => mem.write(match io::stdin().bytes().next() {
                Some(r) => r.unwrap_or(0),
                None => 0,
            }),
            _ => {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn square_bracket() {
        let mut r = Cursor::new("+-><[[]]]++");
        skip_to_closing_parenthesis(&mut r, '[', ']');
        assert_eq!(r.position(), 9);
    }

    #[test]
    fn angle_bracket() {
        let mut r = Cursor::new("<>>");
        skip_to_closing_parenthesis(&mut r, '<', '>');
        assert_eq!(r.position(), 3);
    }
}
