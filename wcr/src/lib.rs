// TODO: comment next line
// #![allow(unused)]

use anyhow::{anyhow, Result};
use argh::FromArgs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(FromArgs, Debug)]
/// Rust wc
pub struct Config {
    /// number of lines
    #[argh(switch, short = 'l')]
    lines: bool,

    /// number of words
    #[argh(switch, short = 'w')]
    words: bool,

    /// number of bytes
    #[argh(switch, short = 'c')]
    bytes: bool,

    /// number of characters
    #[argh(switch, short = 'm')]
    chars: bool,

    /// input file(s)
    #[argh(positional)]
    files: Vec<String>,
}

/// Get Config from command line arguments
pub fn get_args() -> Config {
    let mut config: Config = argh::from_env();

    if !config.lines && !config.words && !config.bytes && !config.chars {
        config.lines = true;
        config.words = true;
        config.bytes = true;
        // config.chars = true;
    }

    if config.files.is_empty() {
        config.files.push(String::from("-"));
    }

    config
}

/// Run wcr
pub fn run(config: Config) -> Result<()> {
    println!("{:#?}", config);

    // let is_one_file = config.files.len() == 1;
    // for filename in config.files {
    //     match open(&filename) {
    //         Err(err) => eprintln!("Failed to open {filename}: {err}"),
    //         Ok(mut file) => {
    //         }
    //     }
    // }
    Ok(())
}

/// Open a file or standard input for reading
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
