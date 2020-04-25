use std::io;
use std::fs;
use std::fs::File;
use std::io::Read;

pub struct Memory {
    ram: Vec<u8>,
    stack: Vec<u8>
}

impl Memory {
    pub fn init() -> Memory {
        Memory {
            ram: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn load(&mut self, file_path: &str) -> Result<(), io::Error> {
        let mut file = File::open(file_path)?;
        file.read_to_end(&mut self.ram);
        Ok(())
    }
}

