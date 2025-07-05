use std::{error::Error, fs};

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // prototype steps:
    /*
    1. grab cmd line args and load in file
    2. convert each line into a mnemonic enum. error check the syntax here!
    3. initialize memory and registers before execution
    4. iterate through the instruction enums and delegate required action accordingly
    */

    let code = fs::read_to_string(config.path).expect("Could not read file");

    print!("{code}");

    // do stuff
    Ok(())
}

#[cfg(test)]
mod tests {
    //use super::*;
}
