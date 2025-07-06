pub struct Config {
    pub path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Expected path"),
        };

        Ok(Config { path })
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operand {
    Immediate(u16),
    Register(Register),
    Indirect(Register),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Register {
    Acc,
    Isp,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Instruction {
    Add(Operand),
    Sub(Operand),
    Mul(Operand),
    Div(Operand),
    Mov(Operand, Register),
    Xor(Operand, Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand, Operand),
    Jmp,
    Jnz,
    Jez,
    Jgz,
    Jlz,
}
