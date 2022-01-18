mod machine;
mod memory;
mod program;
mod set;
mod stack;

use std::process::exit;

use stack::Stack;
use program::Program;
use machine::machine;
use memory::Memory;
use set::*;

fn main() {
    let instructions = [
        LOAD, 0,
        LOAD, 3,
        WRITE,
        LOAD, 5,
        LOAD, 6,
        ADD,
        LOAD, 0,
        READ,
        SUB,
        PRINT,
        END,
    ];

    let mut program = Program::new(Box::new(instructions));
    let mut memory = Memory::new();
    let mut stack = Stack::new();
    machine(&mut program, &mut memory, &mut stack);
}

fn error(message: &str) -> ! {
    eprintln!("ERROR: {}", message);
    exit(1);
}
