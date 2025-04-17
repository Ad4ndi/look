use std::{fs, io, path::{Path, PathBuf}, process};

use crate::{config::Config, search::find_files};

pub struct FileResult {
    pub file: PathBuf,
    pub matched: bool,
    pub count: usize,
}

pub fn run(cfg: Config) -> io::Result<()> {
    let path = Path::new(&cfg.path);
    if path.is_dir() && !cfg.recursive {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Use -r for directories"));
    }

    let files = if path.is_dir() {
        find_files(path)?
    } else {
        vec![path.to_path_buf()]
    };

    let results = files
        .iter()
        .filter_map(|f| process_file(f, &cfg).ok())
        .collect::<Vec<_>>();

    print_results(&results, &cfg);
    check_exit_conditions(&results, &cfg);

    Ok(())
}

fn process_file(path: &Path, cfg: &Config) -> io::Result<FileResult> {
    let content = fs::read_to_string(path)?;
    let matched = content
        .lines()
        .filter(|line| line.contains(&cfg.query) ^ cfg.invert_match)
        .count();

    if !should_skip_printing(cfg) {
        print_matches(&content, path, cfg); }

    Ok(FileResult {
        file: path.to_path_buf(),
        matched: matched > 0,
        count: matched,
    })
}

fn should_skip_printing(cfg: &Config) -> bool {
    cfg.files_with_matches || cfg.files_without_matches || cfg.count }

fn print_matches(content: &str, path: &Path, cfg: &Config) {
    content.lines().enumerate().for_each(|(i, line)| {
        if line.contains(&cfg.query) ^ cfg.invert_match {
            print_match(line, &cfg.query, i + 1, path, cfg);
        }
    });
}

fn print_match(line: &str, query: &str, num: usize, path: &Path, cfg: &Config) {
    let output = match (cfg.only_matching, cfg.line_number) {
        (true, true) => line.find(query).map(|i| format!("{}:{}:{}", path.display(), num, &line[i..i + query.len()])),
        (true, false) => line.find(query).map(|i| format!("{}:{}", path.display(), &line[i..i + query.len()])),
        (false, true) => Some(format!("{}:{}:{}", path.display(), num, line)),
        _ => Some(format!("{}:{}", path.display(), line)),
    };
    output.iter().for_each(|s| println!("{}", s));
}

fn print_results(results: &[FileResult], cfg: &Config) {
    results.iter().for_each(|r| {
        if cfg.files_with_matches && r.matched {
            println!("{}", r.file.display());
        } else if cfg.files_without_matches && !r.matched {
            println!("{}", r.file.display());
        } else if cfg.count && r.matched && !cfg.files_without_matches {
            println!("{}:{}", r.file.display(), r.count);
        }
    });
}

fn check_exit_conditions(results: &[FileResult], cfg: &Config) {
    if cfg.quiet && results.iter().any(|r| r.matched) {
        process::exit(0); }
    if results.iter().all(|r| !r.matched) {
        process::exit(1); }
}
