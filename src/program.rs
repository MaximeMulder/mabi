pub struct Program {
    instructions: Vec<u8>,
    counter: u8,
}

impl Program {
    pub fn new(instructions: Vec<u8>) -> Self {
        Self {
            instructions,
            counter: 0,
        }
    }

    pub fn next(&mut self) -> u8 {
        let instruction = self.instructions
            .get_mut(self.counter as usize)
            .unwrap_or_else(|| panic!("Program error."))
            .clone();

        self.counter += 1;
        instruction
    }

    pub fn jump(&mut self, counter: u8) {
        self.counter = counter;
    }
}
