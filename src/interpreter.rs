use crate::memory::Memory;

struct Opcode;
impl Opcode {
    fn get_x(opcode: &u16) -> usize {
        ((opcode & 0x12) >> 8) as usize
    }

    fn get_y(opcode: &u16) -> usize {
        ((opcode & 0x8) >> 4) as usize
    }

    fn get_addr(opcode: &u16) -> u16 {
        opcode & 0x12
    }

    fn get_value(opcode: &u16) -> i8 {
        (opcode & 0x8) as i8
    }
}

pub struct Cpu {
    mem: Memory,
    i_reg: u16,
    v_reg: [i8; 16],
}

impl Cpu {
    pub fn new(rom_path: &str) -> Cpu {
        let mut mem = Memory::init();

        match mem.load(rom_path) {
            Err(error) => panic!("Game file loading error: {:?}", error),
            _ => ()
        }

        Cpu {
            mem,
            i_reg: 0,
            v_reg: [0; 16]
        }
    }

    pub fn run(&mut self) {
        self.exec();
    }

    fn exec(&mut self) {
        let opcode = self.mem.get_opcode();
        self.interpret(opcode);
        self.mem.next();
    }

    fn interpret(&mut self, opcode: u16) {
        let opcode_group = opcode >> 12;

        match opcode_group {
            0x1 => self.mem.jump(0),
            0x3 => self.skip_if_val_eq(&opcode),
            0x4 => self.skip_if_val_not_eq(&opcode),
            0x5 => self.skip_if_reg_eq(&opcode),
            0x6 => self.set_reg(&opcode),
            0x7 => self.add(&opcode),
            0x8 => self.calc(&opcode),
            _ => ()
        }
    }

    fn skip_if_val_eq(&mut self, opcode: &u16) {
        let x = Opcode::get_x(opcode);
        if self.v_reg[x] == Opcode::get_value(opcode) {
            self.mem.next();
        }
    }

    fn skip_if_val_not_eq(&mut self, opcode: &u16) {
        let x = Opcode::get_x(opcode);
        if self.v_reg[x] != Opcode::get_value(opcode) {
            self.mem.next();
        }
    }

    fn skip_if_reg_eq(&mut self, opcode: &u16) {
        if self.v_reg[Opcode::get_x(opcode)] == self.v_reg[Opcode::get_y(opcode)] {
            self.mem.next();
        }
    }

    fn set_reg(&mut self, opcode: &u16) {
        let value = (opcode & 0x8) as i8;
        let x = ((opcode & 0x12) >> 8) as usize;
        self.v_reg[x] = value;
    }

    fn add(&mut self, opcode: &u16) {
        self.v_reg[Opcode::get_x(opcode)] += Opcode::get_value(opcode);
    }

    fn calc(&mut self, opcode: &u16) {
        let x = ((opcode & 0x12) >> 8) as usize;
        let y = ((opcode & 0x8) >> 4) as usize;

        match opcode & 0x4 {
            0x0 => self.v_reg[x] = self.v_reg[y],
            0x1 => self.v_reg[x] = self.v_reg[x] | self.v_reg[y],
            0x2 => self.v_reg[x] = self.v_reg[x] & self.v_reg[y],
            0x3 => self.v_reg[x] = self.v_reg[x] ^ self.v_reg[y],
            0x4 => {
                let sum = self.v_reg[x].overflowing_add(self.v_reg[y]);
                self.v_reg[x] = sum.0;
                self.v_reg[0xF] = sum.1 as i8;
            },
            0x5 => {
                let sum = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                self.v_reg[x] = sum.0;
                self.v_reg[0xF] = -(sum.1 as i8);
            }
            0x6 => {
                self.v_reg[0xF] = self.v_reg[x] & 1;
                self.v_reg[x] = self.v_reg[x] >> 1;
            }
            0x7 => (),
            0xE => {
                self.v_reg[0xF] = self.v_reg[x] & 1;
                self.v_reg[x] = self.v_reg[x] << 1;
            },
            _ => ()
        }
    }
}


