pub mod cpu;
pub mod parser;
pub mod types;

use crate::cpu::*;
use crate::parser::parse_code;
use crate::types::*;
use std::{error::Error, fs, time::Duration};

use crossterm::{
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut cpu = Cpu::build();
    let code = fs::read_to_string(config.path).expect("Could not read file");
    parse_code(code, &mut cpu);

    loop {
        if poll(Duration::from_millis(500))? {
            let event = read()?;
            if event == Event::Key(KeyCode::Char('s').into()) {
                cpu.execute_one()?;
                println!("{:?}", cpu.registers);
            }

            if event == Event::Key(KeyCode::Char('q').into()) {
                break;
            }
        }
    }

    disable_raw_mode()?;

    println!("{:?}", cpu.registers);

    Ok(())
}
