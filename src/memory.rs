const MEMORY_SIZE: usize = 65536;

pub struct Memory {
    values: [u16; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            values: [0; MEMORY_SIZE],
        }
    }

    pub fn read(&mut self, address: u16) -> u16 {
        self.values[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.values[address as usize] = value;
    }
}
