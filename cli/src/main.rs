use std::env;
use std::process;

use cli;

fn main() {
    // collect argument from terminal and iterate
    // let args: Vec<String> = env::args().collect();

    let config = cli::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = cli::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}

// binary crates
