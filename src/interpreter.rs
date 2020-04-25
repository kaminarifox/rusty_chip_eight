use crate::memory::Memory;

struct Opcode(u8, u8, u8, u8);

struct Cpu {
    v_reg: Vec<i8>,
    i_reg: [u8; 0x10],
    pc: i16,
}

impl Cpu {
    pub fn run(&self, &mem: Memory) {
        loop {
            let opcode = self.read_opcode(mem.ram);
        }
    }

    pub fn read_opcode(&self, ram: &Vec<u8>) -> Opcode {
        return Opcode(ram[pc],ram[pc + 1], ram[pc + 2], ram[pc = 3]);
    }
}
