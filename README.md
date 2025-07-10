# Assembly Simulator in Rust

# CPU Architecture

- 16 bit
- Registers: accumulator, instruction pointer, 8 general purposes (R1-R8)
- 64kb ram

#  Usage
If building from source:
`cargo run -- <file.asm>`

If running from binary:
`.\<.exe> <file.asm>`

Use 's' to step through instructions, 'q' to quit. Interactive terminal is barely implemented, program will just panic once you run out of instructions.

## Instruction Set

**Currently only ADD,SUB,MOV are implemented**

| Mnemonic | Purpose |
| --- | --- |
| ADD <SRC> | Adds the value at SRC register to ACC |
| SUB <SRC> | Subtract the value at SRC from ACC |
| MUL <SRC> | Multiply SRC by ACC |
| DIV <SRC> | Divide ACC by SRC |
| MOV <SRC>, <DEST> | obvious? |
| XOR <A>, <B> | Result goes in ACC |
| AND <A>, <B> | Result goes in ACC |
| OR <A>, <B> | Result goes in ACC |
| NOT <A>, <B> | Result goes in ACC |
| JMP, JNZ, JEZ, JGZ, JLZ | pretty obvious too? |

## Syntax

- Square brackets used to reference value at address
- Operand can be immediate, direct, or indirect
- Labels: LABEL: (must be only thing on line)
- Comments: // for single line comment
- One instruction per line

e.g:

```nasm
// This is a comment
MOV 0, ACC
ADD 5
ADD 3
SUB 2
DIV [R2]
```
