mod file;
mod machine;
mod memory;
mod program;
mod set;
mod stack;

use std::env;

use file::read;
use machine::machine;
use memory::Memory;
use program::Program;
use stack::Stack;

fn main() {
    let arguments = env::args().collect::<Box<[_]>>();
    if arguments.len() != 2 {
        panic!("Arguments error.");
    }

    let instructions = read(&arguments[1]);
    let mut program = Program::new(instructions);
    let mut memory = Memory::new();
    let mut stack = Stack::new();
    machine(&mut program, &mut stack, &mut memory);
}
