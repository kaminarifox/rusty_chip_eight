use std::io;
use std::fs::File;
use std::io::Read;
use crate::interpreter::Cpu;

pub struct Memory {
    ram: Vec<u8>,
    stack: Vec<u8>,
    pc: usize
}

impl Memory {
    pub fn init() -> Memory {
        Memory {
            ram: Vec::new(),
            stack: Vec::new(),
            pc: 0
        }
    }

    pub fn load(&mut self, file_path: &str) -> Result<(), io::Error> {
        let mut file = File::open(file_path)?;
        file.read_to_end(&mut self.ram);
        Ok(())
    }

    pub fn get_opcode(&mut self) -> u16 {
        let high_bits: u16 = (self.ram[self.pc]) as u16;
        let low_bits: u16 = self.ram[self.pc + 1] as u16;

        high_bits << 8 + low_bits
    }

    pub fn jump(&mut self, jmp_addr: usize) {
        self.pc = jmp_addr;
    }

    pub fn next(&mut self) {
        self.pc += 2;
    }
}

