const BUFFER_SIZE: usize = 64 * 32;

pub struct Screen {
    buffer: [bool; BUFFER_SIZE],
}

impl Screen {
    pub fn clear() {
        self.buffer = [false; BUFFER_SIZE];
    }

    pub fn draw(&self) {
        print!("\x1B[2J"); // Clear screen
        for (i, pixel) in self.buffer.iter().enumerate() {
            match pixel {
                true => print!("#"),
                false => print!(" "),
            }

            if i == 0x20 {
                print!("\n");
            }
        }
    }
}
