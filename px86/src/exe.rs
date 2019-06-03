use crate::emu;
use std::error::Error;
use std::io::Read;

pub fn execute<I: Read>(input: &mut I) -> Result<(), Box<Error>> {
    let mut emu = emu::Emulator::new();
    emu.load(input);
    emu.run();
    emu.dump();
    Ok(())
}
