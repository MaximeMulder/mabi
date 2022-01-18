use crate::error;

pub struct Stack {
    values: Vec<u16>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    pub fn push(&mut self, value: u16) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> u16 {
        self.values.pop().unwrap_or_else(|| error("Stack error."))
    }
}
