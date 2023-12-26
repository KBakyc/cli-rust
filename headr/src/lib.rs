use anyhow::{anyhow, Result};
use argh::{FromArgValue, FromArgs};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

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
    #[argh(positional, greedy)]
    files: Vec<String>,
}

#[derive(Debug)]
struct FileList(Vec<String>);

impl FromStr for FileList {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FileList(if s.is_empty() {
            vec!["-".to_string()]
        } else {
            s.split_whitespace().map(|s| s.to_string()).collect()
        }))
    }
}

pub fn get_args() -> Config {
    let mut config: Config = argh::from_env();

    if config.files.is_empty() {
        config.files.push(String::from("-"));
    }

    config
}

pub fn run(config: Config) -> Result<()> {
    println!("{:#?}", config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {}
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

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
