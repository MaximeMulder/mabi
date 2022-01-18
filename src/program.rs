use crate::error;

pub struct Program {
    instructions: Box<[u16]>,
    counter: u16,
}

impl Program {
    pub fn new(instructions: Box<[u16]>) -> Self {
        Self {
            instructions,
            counter: 0,
        }
    }

    pub fn next(&mut self) -> u16 {
        let counter = self.counter as usize;
        if counter >= self.instructions.len() {
            error("Program error.");
        }

        let instruction = self.instructions[counter];
        self.counter += 1;
        instruction
    }

    pub fn jump(&mut self, counter: u16) {
        self.counter = counter;
    }
}
