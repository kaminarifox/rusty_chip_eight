use crate::memory;

fn main() {
    let mut mem = memory::Memory::init();
    mem.load("test").unwrap()
}

fn main_loop() {

}
