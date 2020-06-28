use crate::memory::Memory;
use crate::graphics::Screen;
use std::ops::BitAnd;

#[derive(Debug)]
struct Opcode(u8, u8);

impl Opcode {
    fn as_int(&self) -> u32 {
        self.0.to_be_bytes()
    }

    fn as_address(&self) -> u16 {
        return u16::from_be_bytes([self.0 >> 4, self.1])
    }
}

struct Cpu {
    v_reg: Vec<i8>,
    i_reg: u16  ,
    pc: i16,
    stack: Vec<i16>,
    ram: Memory,
}

impl Cpu {
    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.exec_opcode(opcode);
        }
    }

    fn exec_opcode(&mut self, opcode: Opcode) {
        match opcode {
            // Opcode(0x0, 0x0, 0xE, 0x0) => Screen::clear(),
            // Opcode(0x1, ) => self.jmp(opcode),
            // Opcode(0x3, _, _, _) => if self.v_reg[opcode.1] == u16::from_le_bytes(opcode.2, opcode.3)
            _ => println!("{:?}", opcode),
        }
    }

    fn read_opcode(&self) -> Opcode {
        // return Opcode(
            // self.ram[pc],
            // self.ram[pc + 1],
            // self.ram[pc + 2],
            // self.ram[pc + 3],
        // );
    }


    fn cls(&mut self) {

    }

    fn ret(&mut self) {
        let addr = self.stack.pop();
        match addr {
            Ok(&value) => self.pc,
            Err(e) => println!("Error pop from stack: {:?}", e),
            _ => {},
        };
    }

    fn jmp(&mut self, &opcode: Opcode) {
        self.pc = opcode.as_address();
    }

    fn jsr(&mut self) {

    }

    fn skeq(&mut self) {

    }



}
