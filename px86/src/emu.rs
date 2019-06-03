use std::error::Error;
use std::io::Read;

const MEMORY_SIZE: usize = 1024 * 1024;

struct Register {
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esp: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
}

impl Register {
    pub fn new() -> Self {
        Self {
            eax: 0,
            ecx: 0,
            edx: 0,
            ebx: 0,
            esp: 0x7c00,
            ebp: 0,
            esi: 0,
            edi: 0,
        }
    }

    pub fn set(&mut self, num: u8, val: u32) {
        match num {
            0 => self.eax = val,
            1 => self.ecx = val,
            2 => self.edx = val,
            3 => self.ebx = val,
            4 => self.esp = val,
            5 => self.ebp = val,
            6 => self.esi = val,
            7 => self.edi = val,
            _ => {}
        }
    }

    pub fn dump(&self) {
        println!("EAX = 0x{:08x}", self.eax);
        println!("ECX = 0x{:08x}", self.ecx);
        println!("EDX = 0x{:08x}", self.edx);
        println!("EBX = 0x{:08x}", self.ebx);
        println!("ESP = 0x{:08x}", self.esp);
        println!("EBP = 0x{:08x}", self.ebp);
        println!("ESI = 0x{:08x}", self.esi);
        println!("EDI = 0x{:08x}", self.edi);
    }
}

pub struct Emulator {
    regs: Register,
    mem: [u8; MEMORY_SIZE],
    eip: usize,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            regs: Register::new(),
            mem: [0; MEMORY_SIZE],
            eip: 0,
        }
    }

    pub fn load<I: Read>(&mut self, f: &mut I) {
        f.read(&mut self.mem[0..0x200]).unwrap();
    }

    pub fn instruction(&mut self, code: u8) -> Result<(), Box<Error>> {
        match code {
            0xb8..=0xbf => self.mov_r32_imm32(),
            0xeb => self.short_jump(),
            _ => return Err(From::from("Not Implemented")),
        }

        Ok(())
    }

    pub fn run(&mut self) {
        while self.eip < MEMORY_SIZE {
            let code = self.get_code8(0);
            println!("EIP = 0x{:08x}, Code = 0x{:02x}", self.eip, code);
            if let Err(e) = self.instruction(code) {
                eprintln!("Error: {}", e);
                return;
            }
            if self.eip == 0 {
                println!("end of program.");
                return;
            }
        }
    }

    pub fn dump(&self) {
        self.regs.dump();
        println!("EIP = 0x{:08x}", self.eip);
    }

    fn get_code8(&self, index: usize) -> u8 {
        self.mem[self.eip + index]
    }

    fn get_sign_code8(&self, index: usize) -> i8 {
        self.get_code8(index) as i8
    }

    fn get_code32(&self, index: usize) -> u32 {
        let mut code = 0_u32;
        for i in 0..4 {
            code |= (self.get_code8(index + i) as u32) << (i * 8);
        }
        code
    }

    fn mov_r32_imm32(&mut self) {
        let reg = self.get_code8(0) - 0xb8;
        let val = self.get_code32(1);
        self.regs.set(reg, val);
        self.eip += 5;
    }

    fn short_jump(&mut self) {
        let diff = self.get_sign_code8(1);
        self.eip = (self.eip as i32 + diff as i32 + 2) as usize;
    }
}
