mod config;
mod search;
mod grep;

use std::process;

fn main() {
    match config::parse_args() {
        Ok(cfg) => grep::run(cfg).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            process::exit(1);
        }),
        Err(e) => {
            eprintln!("Error: {}", e);
            config::print_help();
            process::exit(1);
        }
    }
}
