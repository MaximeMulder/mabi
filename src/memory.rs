pub struct Memory {
    values: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    pub fn read(&mut self, address: u8) -> u8 {
        while address as usize >= self.values.len() {
            self.values.push(0);
        }

        self.values[address as usize]
    }

    pub fn write(&mut self, address: u8, value: u8) {
        while address as usize >= self.values.len() {
            self.values.push(0);
        }

        self.values[address as usize] = value;
    }
}
