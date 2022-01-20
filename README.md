# MABI

## Introduction

Mabi is a small virtual machine I am writing to learn more about this domain.

My goals with Mabi are to write a minimalist abstract virtual machine with an instruction set that is independent of any physical architecture or language design.

Mabi is not designed to be very optimized, but rather to have a simple and readable codebase.

## Architecture

The architecture of Mabi is subject to change.

Mabi is a load-store, stack machine composed of three parts: the program, the stack, and the memory.

The program is composed of one-byte instructions, which can optionally have four-byte parameters, written in big-endian.

The stack holds four-byte values produced and consumed by the instructions.

The memory can contain four-byte values load and stored by the instructions.

Mabi uses an Harvard architecture, with a different addresses for the program and the memory. This ensures the program cannot modify itself or generate code, allowing safe jumps to arbitrary addresses.

| Opcode | Name  | Parameters | Input      | Output |
|--------|-------|------------|------------|--------|
| 0x00   | NOP   | -          | -          | -      |
| 0x01   | CONST | val        | -          | val    |
| 0x02   | LOAD  | -          | addr       | val    |
| 0x03   | STORE | -          | addr, val  | -      |
| 0x04   | AND   | -          | a, b       | res    |
| 0x05   | OR    | -          | a, b       | res    |
| 0x06   | XOR   | -          | a, b       | res    |
| 0x07   | NOT   | -          | a          | res    |
| 0x08   | ADD   | -          | a, b       | res    |
| 0x09   | SUB   | -          | a, b       | res    |
| 0x0A   | MUL   | -          | a, b       | res    |
| 0x0B   | DIV   | -          | a, b       | res    |
| 0x0C   | REM   | -          | a, b       | res    |
| 0x0D   | JUMP  | -          | addr       | -      |
| 0x0E   | IF    | -          | addr, cond | -      |
| 0x0F   | PRINT | -          | val        | -      |
| 0x10   | END   | -          | -          | -      |
