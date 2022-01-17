pub struct Stack {
    values: Vec<u8>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    pub fn push(&mut self, value: u8) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> u8 {
        self.values.pop().unwrap_or_else(|| panic!("Stack error"))
    }
}
