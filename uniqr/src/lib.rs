// TODO: comment next line
// #![allow(unused)]

use anyhow::{anyhow, Result};
use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

/// Standard input alias
const STDIN_NAME: &str = "-";

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

/// Get Config from command line arguments
pub fn get_args() -> Config {
    let matches = Command::new("uniqr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value(STDIN_NAME),
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .help("Output file"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .help("Show counts"),
        )
        .get_matches();

    Config {
        in_file: matches.get_one("in_file").cloned().unwrap(),
        out_file: matches.get_one("out_file").cloned(),
        count: matches.get_flag("count"),
    }
}

/// Run program
pub fn run(config: Config) -> Result<()> {
    let mut file = open(&config.in_file).map_err(|e| anyhow!("{}: {e}", config.in_file))?;

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |num: u32, text: &str| -> Result<()> {
        if num > 0 {
            if config.count {
                write!(out_file, "{num:>4} {text}")?;
            } else {
                write!(out_file, "{text}")?;
            }
        };
        Ok(())
    };

    let mut previous = String::new();
    let mut count = 0u32;
    loop {
        let mut line = String::new();
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line;
            count = 0;
        }

        count += 1;
    }
    print(count, &previous)?;

    Ok(())
}

/// Open a file or standard input for reading
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        STDIN_NAME => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
