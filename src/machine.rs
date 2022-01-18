use crate::program::Program;
use crate::set::*;
use crate::stack::Stack;
use crate::memory::Memory;

pub fn machine(program: &mut Program, memory: &mut Memory, stack: &mut Stack) {
    loop {
        match program.next() {
            NOP => {},
            CONST => {
                let value = program.next();
                stack.push(value);
            },
            LOAD => {
                let address = stack.pop();
                let value = memory.load(address);
                stack.push(value);
            },
            STORE => {
                let value = stack.pop();
                let address = stack.pop();
                memory.store(address, value);
            },
            AND => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a & b);
            },
            OR => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a | b);
            },
            XOR => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a ^ b);
            },
            NOT => {
                let a = stack.pop();
                stack.push(!a);
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
                if b == 0 {
                    panic!("Division by zero.");
                }

                stack.push(a / b);
            },
            REM => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a % b);
            },
            JUMP => {
                let counter = stack.pop();
                program.jump(counter);
            },
            IF => {
                let counter = stack.pop();
                let condition = stack.pop();
                if condition == 0 {
                    program.jump(counter);
                }
            },
            PRINT => {
                println!("{}", stack.pop());
            },
            END => {
                return;
            },
            instruction => {
                panic!("Unknown instruction 0x{:x}.", instruction);
            },
        }
    }
}
