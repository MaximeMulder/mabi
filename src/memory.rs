const MEMORY_SIZE: usize = 65536;

pub struct Memory {
    values: [u32; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            values: [0; MEMORY_SIZE],
        }
    }

    pub fn load(&mut self, address: usize) -> u32 {
        self.values[address]
    }

    pub fn store(&mut self, address: usize, value: u32) {
        self.values[address] = value;
    }
}
