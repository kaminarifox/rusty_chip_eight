use crate::interpreter::Cpu;

mod interpreter;
mod memory;

fn main() {
    let mut cpu = Cpu::new("/home/ei/Learning/Rust/rusty_chip_eight/roms/maze.ch8");
    cpu.run();
}
