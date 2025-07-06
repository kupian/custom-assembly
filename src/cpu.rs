use crate::types::*;
use std::{collections::HashMap, error::Error};

#[derive(Debug, PartialEq, Eq)]
pub struct Cpu {
    pub instructions: Vec<Instruction>,
    pub labels: HashMap<String, usize>,
    pub memory: Vec<u16>,
    pub registers: [u16; 11],
}

impl Cpu {
    pub fn build() -> Self {
        Cpu {
            instructions: Vec::new(),
            labels: HashMap::new(),
            memory: Vec::new(),
            registers: [0; 11],
        }
    }

    pub fn write_reg(&mut self, reg: Register, val: u16) {
        self.registers[reg as usize] = val;
    }

    pub fn read_reg(&self, reg: Register) -> u16 {
        self.registers[reg as usize]
    }

    fn value_from_operand(&self, op: Operand) -> Result<u16, Box<dyn Error>> {
        match op {
            Operand::Immediate(val) => Ok(val),
            Operand::Register(reg) => Ok(self.read_reg(reg)),
            Operand::Indirect(reg) => {
                let addr = self.read_reg(reg) as usize;
                self.memory
                    .get(addr)
                    .copied()
                    .ok_or_else(|| format!("Invalid memory access at {addr}").into())
            }
        }
    }

    fn add(&mut self, op: Operand) -> Result<(), Box<dyn Error>> {
        let value_to_add = self.value_from_operand(op)?;

        let total_value = self
            .read_reg(Register::Acc)
            .checked_add(value_to_add)
            .ok_or("Overflow occurred in accumulator")?;

        self.write_reg(Register::Acc, total_value);
        Ok(())
    }

    fn sub(&mut self, op: Operand) -> Result<(), Box<dyn Error>> {
        let value_to_sub = self.value_from_operand(op)?;

        let total_value = self
            .read_reg(Register::Acc)
            .checked_sub(value_to_sub)
            .ok_or("Underflow occurred in accumulator")?;

        self.write_reg(Register::Acc, total_value);
        Ok(())
    }

    fn mov(&mut self, src: Operand, dest: Register) -> Result<(), Box<dyn Error>> {
        let value_to_mov = self.value_from_operand(src)?;

        self.write_reg(dest, value_to_mov);
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        for i in 0..self.instructions.len() {
            let instruction = &self.instructions[i];
            match instruction {
                Instruction::Add(op) => self.add(*op)?,
                Instruction::Sub(op) => self.sub(*op)?,
                Instruction::Mov(src, dest) => self.mov(*src, *dest)?,
                _ => return Err("Unsupported instruction: {instruction:?}".into()),
            }
        }
        Ok(())
    }
}
