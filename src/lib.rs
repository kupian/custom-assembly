pub mod cpu;
pub mod parser;
pub mod types;

use crate::cpu::*;
use crate::parser::parse_code;
use crate::types::*;
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut cpu = Cpu::build();
    let code = fs::read_to_string(config.path).expect("Could not read file");

    parse_code(code, &mut cpu);

    cpu.execute()?;

    println!("{cpu:?}");

    Ok(())
}
