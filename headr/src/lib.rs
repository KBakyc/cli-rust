// TODO: remove next line
// #![allow(unused)]

use anyhow::{anyhow, Result};
use argh::FromArgs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(FromArgs, Debug)]
/// Rust head
pub struct Config {
    /// number of lines
    #[argh(option, short = 'n', default = "10")]
    lines: usize,

    /// number of bytes
    #[argh(option, short = 'c')]
    bytes: Option<usize>,

    /// input file(s)
    #[argh(positional)]
    files: Vec<String>,
}

pub fn get_args() -> Config {
    let mut config: Config = argh::from_env();

    if config.lines < 1 {
        panic!("the argument '--lines <LINES>' cannot be less than 1");
    }

    if let Some(bytes) = config.bytes {
        if bytes < 1 {
            panic!("the argument '--bytes <BYTES>' cannot be less than 1");
        }
        if config.lines != 10 {
            // default value of lines is changed
            panic!("the argument '--lines <LINES>' cannot be used with '--bytes <BYTES>'");
        }
    };

    if config.files.is_empty() {
        config.files.push(String::from("-"));
    }

    config
}

pub fn run(config: Config) -> Result<()> {
    let is_one_file = config.files.len() == 1;
    let mut is_first_file = true;
    for filename in config.files {
        if !is_one_file {
            if is_first_file {
                is_first_file = false;
            } else {
                println!();
            }
            println!("==> {filename} <==");
        }
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(mut file) => {
                if let Some(bytes) = config.bytes {
                    print_bytes(&mut file, bytes);
                } else {
                    print_lines_with_original_eol(&mut file, config.lines)?;
                }
            }
        }
    }
    Ok(())
}

/// print lines with original end of line characters
fn print_lines_with_original_eol(file: &mut dyn BufRead, lines: usize) -> Result<()> {
    for _ in 0..lines {
        let mut buffer = String::new();
        let bytes_read = file.read_line(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        print!("{buffer}");
    }
    Ok(())
}

/// print bytes
fn print_bytes(file: &mut dyn BufRead, bytes: usize) {
    let mut handle = io::Read::take(file, bytes as u64);
    let mut buffer = vec![0; bytes];
    while handle.read_exact(&mut buffer).is_ok() {
        print!("{}", String::from_utf8_lossy(&buffer));
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[allow(dead_code)]
fn parse_positive_int(val: &str) -> Result<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(anyhow!(val.to_string())),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);
    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
