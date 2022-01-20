pub struct Stack {
    values: Vec<u32>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    pub fn push(&mut self, value: u32) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> u32 {
        self.values.pop().unwrap_or_else(|| panic!("Stack error."))
    }
}
