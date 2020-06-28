use crate::memory::Memory;

pub struct Cpu {

}

impl Cpu {
    pub fn exec(mem: &mut Memory) {
        let opcode = mem.get_opcode();
        println!("{:?}", opcode);
    }
}


