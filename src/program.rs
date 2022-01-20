pub struct Program {
    instructions: Box<[u8]>,
    counter: usize,
}

impl Program {
    pub fn new(instructions: Box<[u8]>) -> Self {
        Self {
            instructions,
            counter: 0,
        }
    }

    pub fn next(&mut self) -> u8 {
        if self.counter >= self.instructions.len() {
            panic!("Program error.");
        }

        let instruction = self.instructions[self.counter];
        self.counter += 1;
        instruction
    }

    pub fn jump(&mut self, counter: usize) {
        self.counter = counter;
    }
}
