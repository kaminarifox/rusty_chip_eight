mod interpreter;
mod memory;

fn main() {
    let mut mem = memory::Memory::init();
    match mem.load("/home/ei/Learning/Rust/rusty_chip_eight/roms/maze.ch8") {
        Err(error) => panic!("Game file loading error: {:?}", error),
        _ => ()
    }

    mem.run();
}
