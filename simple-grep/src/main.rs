use anyhow::Result;
use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};

// Simple grep program
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Regex to search for in file
    #[clap(value_parser)]
    regex: String,

    // File to search regex in
    #[clap(value_parser)]
    file: PathBuf,
}

fn read_lines<F>(filename: F) -> Result<io::Lines<io::BufReader<File>>>
where
    F: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file: &PathBuf = &args.file;

    if !file.is_file() {
        println!("'{}' is not a file!", file.display());
        std::process::exit(1);
    }

    let regex = match Regex::new(&args.regex) {
        Ok(r) => r,
        Err(_) => {
            println!("'{}' is not valid regex!", args.regex);
            std::process::exit(2);
        }
    };

    let lines = match read_lines(file) {
        Ok(l) => l,
        Err(_) => {
            println!("Could not read file '{}'", file.display());
            std::process::exit(3);
        }
    };

    for line in lines.flatten() {
        if regex.is_match(&line) {
            println!("{}", line);
        }
    }

    Ok(())
}
