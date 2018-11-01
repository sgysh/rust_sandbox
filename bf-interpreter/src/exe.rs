use mem;
use std::error::Error;
use std::io;
use std::io::{Read, Write, Seek, SeekFrom};

fn skip_to_closing_parenthesis<R: Read>(r: &mut R, open: char, close: char) -> Result<(), Box<Error>> {
    let mut nest = 1;
    let mut b = [0_u8; 1];

    while nest != 0 {
        r.read(&mut b)?;
        let c = b[0];

        if c == open as u8 {
            nest += 1;
        } else if c == close as u8 {
            nest -= 1;
        }
    }

    Ok(())
}

/// ## Example
/// ```
/// let mut input = std::io::Cursor::new("++++++++[>++++++++<-]>+.+.+.");
/// let mut output = Vec::new();
///
/// assert!(bf_interpreter::execute(&mut output, &mut input).is_ok());
/// assert_eq!(output, b"ABC");
/// ```
pub fn execute<O: Write, I: Read + Seek>(output: &mut O, input: &mut I) -> Result<(), Box<Error>> {
    let mut mem: mem::Memory = mem::Memory::init(30000);
    let mut stack = Vec::new();
    let mut b = [0_u8; 1];

    while input.read(&mut b)? != 0 {
        let c = b[0] as char;
        match c {
            '+' => mem.add(1),
            '-' => mem.sub(1),
            '>' => mem.next(),
            '<' => mem.prev(),
            '[' => {
                if mem.val() != 0 {
                    let p = input.seek(SeekFrom::Current(0))?;
                    stack.push(p);
                } else {
                    skip_to_closing_parenthesis(input, '[', ']')?;
                }
            },
            ']' => {
                let p = stack.pop().ok_or("No open parenthesis".to_owned())?;
                input.seek(SeekFrom::Start(p - 1))?;
            },
            '.' => {write!(output, "{}", mem.val() as char)?;},
            ',' => mem.write(match io::stdin().bytes().next() {
                Some(r) => r.unwrap_or(0),
                None => 0,
            }),
            _ => {},
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn square_bracket() {
        let mut r = Cursor::new("+-><[[]]]++");
        assert!(skip_to_closing_parenthesis(&mut r, '[', ']').is_ok());
        assert_eq!(r.position(), 9);
    }

    #[test]
    fn angle_bracket() {
        let mut r = Cursor::new("<>>");
        assert!(skip_to_closing_parenthesis(&mut r, '<', '>').is_ok());
        assert_eq!(r.position(), 3);
    }
}
