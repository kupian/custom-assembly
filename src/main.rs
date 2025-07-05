use custom_assembly::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });

    if let Err(e) = custom_assembly::run(config) {
        println!("App error: {e}");
        process::exit(1);
    }
}
