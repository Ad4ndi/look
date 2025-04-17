use std::{env, process};

#[derive(Default)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
    pub quiet: bool,
    pub only_matching: bool,
    pub count: bool,
    pub invert_match: bool,
    pub line_number: bool,
    pub recursive: bool,
    pub files_with_matches: bool,
    pub files_without_matches: bool,
}

pub fn parse_args() -> Result<Config, String> {
    let mut args = env::args().skip(1);
    let mut cfg = Config::default();
    let mut positional = Vec::new();

    while let Some(arg) = args.next() {
        if arg == "-h" {
            print_help();
            process::exit(0);
        }
        if arg.starts_with('-') {
            arg.chars().skip(1).for_each(|c| match c {
                'i' => cfg.ignore_case = true,
                'q' => cfg.quiet = true,
                'o' => cfg.only_matching = true,
                'c' => cfg.count = true,
                'v' => cfg.invert_match = true,
                'n' => cfg.line_number = true,
                'r' => cfg.recursive = true,
                'l' => cfg.files_with_matches = true,
                'L' => cfg.files_without_matches = true,
                _ => (),
            });
        } else {
            positional.push(arg);
        }
    }

    if cfg.files_with_matches && cfg.files_without_matches {
        return Err("Can't use -l with -L".into());
    }
    if positional.len() != 2 {
        return Err("Need query and path".into());
    }

    cfg.query = positional[0].clone();
    cfg.path = positional[1].clone();
    Ok(cfg)
}

pub fn print_help() {
    println!("Usage: grep <query> <path> [options]");
    println!("Options:");
    println!("  -i  Ignore case");
    println!("  -q  Quiet mode (exit with 0 if match found)");
    println!("  -o  Show only matching part");
    println!("  -c  Count matching lines");
    println!("  -v  Invert match (show non-matching lines)");
    println!("  -n  Show line numbers");
    println!("  -r  Recursive directory search");
    println!("  -l  Show only names of files with matches");
    println!("  -L  Show only names of files without matches");
    println!("  -h  Show this help");
}
