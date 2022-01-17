mod memory;
mod program;
mod stack;

use stack::Stack;
use program::Program;
use memory::Memory;

const NOP:   u8 = 0;
const LOAD:  u8 = 1;
const READ:  u8 = 2;
const WRITE: u8 = 3;
const ADD:   u8 = 4;
const SUB:   u8 = 5;
const MUL:   u8 = 6;
const DIV:   u8 = 7;
const REM:   u8 = 8;
const JUMP:  u8 = 9;
const IF:    u8 = 10;
const PRINT: u8 = 11;
const END:   u8 = 12;

fn main() {
    let mut program = Program::new(vec![
        LOAD, 3,
        WRITE, 0,
        LOAD, 5,
        LOAD, 6,
        ADD,
        READ, 0,
        SUB,
        PRINT,
        END,
    ]);

    let mut memory = Memory::new();
    let mut stack = Stack::new();
    loop {
        let isa = program.next();
        match isa {
            NOP => {},
            LOAD => {
                stack.push(program.next());
            },
            READ => {
                let address = program.next();
                let value = memory.read(address);
                stack.push(value);
            },
            WRITE => {
                let address = program.next();
                let value = stack.pop();
                memory.write(address, value);
            },
            ADD => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a + b);
            },
            SUB => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a - b);
            },
            MUL => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a * b);
            },
            DIV => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a / b);
            },
            REM => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a % b);
            },
            JUMP => {
                let counter = program.next();
                program.jump(counter);
            },
            IF => {
                if stack.pop() == 0 {
                    let counter = program.next();
                    program.jump(counter);
                }
            },
            PRINT => {
                println!("{}", stack.pop());
            },
            END => return,
            _ => panic!("Unknown instruction"),
        }
    }
}
