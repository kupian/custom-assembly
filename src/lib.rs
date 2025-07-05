use std::{collections::HashMap, error::Error, fs};

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

#[derive(Debug, PartialEq, Eq)]
enum Operand {
    Immediate(u16),
    Register(Register),
    Indirect(Register),
}

#[derive(Debug, PartialEq, Eq)]
enum Register {
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

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
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

#[derive(Debug, PartialEq, Eq)]
struct Cpu {
    instructions: Vec<Instruction>,
    labels: HashMap<String, usize>,
    memory: Vec<u16>,
    registers: [u16; 11],
}

impl Cpu {
    fn build() -> Self {
        Cpu {
            instructions: Vec::new(),
            labels: HashMap::new(),
            memory: Vec::new(),
            registers: [0; 11],
        }
    }

    fn write_reg(&mut self, reg: Register, val: u16) {
        self.registers[reg as usize] = val;
    }

    fn read_reg(&self, reg: Register) -> u16 {
        self.registers[reg as usize]
    }
}

enum Line {
    Comment,
    Label(String),
    Instruction(Instruction),
    Empty,
}

fn parse_label(line: &str) -> Option<String> {
    let trimmed_line = line.trim();
    if let Some(label) = trimmed_line.strip_suffix(':') {
        if !label.contains(char::is_whitespace) {
            return Some(label.to_string());
        }
    }
    None
}

fn parse_register(reg: &str) -> Option<Register> {
    let trimmed_reg = reg.trim();
    match trimmed_reg.to_lowercase().as_str() {
        "r0" => Some(Register::R0),
        "r1" => Some(Register::R1),
        "r2" => Some(Register::R2),
        "r3" => Some(Register::R3),
        "r4" => Some(Register::R4),
        "r5" => Some(Register::R5),
        "r6" => Some(Register::R6),
        "r7" => Some(Register::R7),
        "r8" => Some(Register::R8),
        _ => None,
    }
}

fn parse_operand(operand: &str) -> Option<Operand> {
    let trimmed_op = operand.trim();
    if let Some(indirect) = trimmed_op.strip_prefix("[") {
        if let Some(indirect) = indirect.strip_suffix("]") {
            if let Some(register) = parse_register(indirect) {
                return Some(Operand::Indirect(register));
            }
        }
    }

    if let Some(register) = parse_register(operand) {
        return Some(Operand::Register(register));
    }

    if operand.parse::<u16>().is_ok() {
        return Some(Operand::Immediate(operand.parse::<u16>().unwrap()));
    }

    None
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let mut words = line.split_whitespace();
    if let Some(opcode) = words.next() {
        return match opcode.trim().to_lowercase().as_str() {
            "add" => words.next().and_then(parse_operand).map(Instruction::Add),
            "sub" => words.next().and_then(parse_operand).map(Instruction::Sub),
            "mov" => {
                let src = words.next().and_then(parse_operand);
                let dst = words.next().and_then(parse_register);
                match (src, dst) {
                    (Some(src), Some(dst)) => Some(Instruction::Mov(src, dst)),
                    _ => None,
                }
            }
            _ => None,
        };
    }
    None
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // prototype steps:
    /*
    1. grab cmd line args and load in file
    2. convert each line into a mnemonic enum. error check the syntax here!
    3. initialize memory and registers before execution
    4. iterate through the instruction enums and delegate required action accordingly
    */

    let mut cpu = Cpu::build();

    let code = fs::read_to_string(config.path).expect("Could not read file");

    for line in code.lines() {
        let line = line.trim();
        let parsed = if line.is_empty() {
            Line::Empty
        } else if line.starts_with("//") {
            Line::Comment
        } else if let Some(label) = parse_label(line) {
            Line::Label(label)
        } else if let Some(instruction) = parse_instruction(line) {
            Line::Instruction(instruction)
        } else {
            panic!("Failed to parse line: {line}")
        };

        match parsed {
            Line::Label(label) => {
                cpu.labels.insert(label, cpu.instructions.len());
            }
            Line::Instruction(instruction) => {
                cpu.instructions.push(instruction);
            }
            _ => {}
        }
    }

    println!("{cpu:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label() {
        assert_eq!(parse_label(" DONE:"), Some(String::from("DONE")));
    }

    #[test]
    fn test_parse_register() {
        assert_eq!(parse_register("r4"), Some(Register::R4));
    }

    #[test]
    fn test_parse_immediate_operand() {
        assert_eq!(parse_operand("16"), Some(Operand::Immediate(16)));
    }

    #[test]
    fn test_parse_register_operand() {
        assert_eq!(parse_operand("r8"), Some(Operand::Register(Register::R8)));
    }

    #[test]
    fn test_parse_indirect_operand() {
        assert_eq!(parse_operand("[r5]"), Some(Operand::Indirect(Register::R5)));
    }

    #[test]
    fn test_parse_instruction_add() {
        assert_eq!(
            parse_instruction("ADD r2"),
            Some(Instruction::Add(Operand::Register(Register::R2)))
        );
    }
}
