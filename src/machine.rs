use crate::program::Program;
use crate::set::*;
use crate::stack::Stack;
use crate::memory::Memory;

pub fn machine(program: &mut Program, stack: &mut Stack, memory: &mut Memory) {
    loop {
        match program.next() {
            NOP => {},
            CONST => {
                let a = program.next();
                let b = program.next();
                let c = program.next();
                let d = program.next();
                let value = (a as u32) << 24 | (b as u32) << 16 | (c as u32) << 8 | d as u32;
                stack.push(value);
            },
            LOAD => {
                let address = stack.pop();
                let value = memory.load(address as usize);
                stack.push(value);
            },
            STORE => {
                let value = stack.pop();
                let address = stack.pop();
                memory.store(address as usize, value);
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
                program.jump(counter as usize);
            },
            IF => {
                let counter = stack.pop();
                let condition = stack.pop();
                if condition == 0 {
                    program.jump(counter as usize);
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
