// TODO: comment next line
// #![allow(unused)]

use anyhow::Result;
use argh::FromArgs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Standard input alias
const STDIN_NAME: &str = "-";

/// Rust wc
#[derive(FromArgs, Debug)]
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

#[derive(Debug, PartialEq, Default)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}
impl FileInfo {
    fn add(&self, info: &FileInfo) -> FileInfo {
        FileInfo {
            num_lines: self.num_lines + info.num_lines,
            num_words: self.num_words + info.num_words,
            num_bytes: self.num_bytes + info.num_bytes,
            num_chars: self.num_chars + info.num_chars,
        }
    }
}

/// Get Config from command line arguments
pub fn get_args() -> Config {
    let mut config: Config = argh::from_env();

    if !config.lines && !config.words && !config.bytes && !config.chars {
        config.lines = true;
        config.words = true;
        config.bytes = true;
        // config.chars = true;
    } else if config.bytes && config.chars {
        panic!("the argument '--chars' cannot be used with '--bytes'");
    }

    if config.files.is_empty() {
        config.files.push(String::from(STDIN_NAME));
    }

    config
}

/// Run wcr
pub fn run(config: Config) -> Result<()> {
    // println!("{:#?}", config);

    let is_one_file = config.files.len() == 1;
    let mut sum_info = FileInfo::default();
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                let info = count(file)?;
                print_info_line(&info, &config, filename);
                sum_info = sum_info.add(&info);
            }
        }
    }
    if !is_one_file {
        print_info_line(&sum_info, &config, "total");
    }
    Ok(())
}

/// Open a file or standard input for reading
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        STDIN_NAME => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    loop {
        let mut line = String::new();
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += bytes;
        num_chars += line.chars().count();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn print_info_line(info: &FileInfo, config: &Config, filename: &str) {
    println!(
        "{}{}{}{}{}",
        format_field(info.num_lines, config.lines),
        format_field(info.num_words, config.words),
        format_field(info.num_bytes, config.bytes),
        format_field(info.num_chars, config.chars),
        if filename == STDIN_NAME {
            String::new()
        } else {
            format!(" {}", filename)
        }
    );
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{value:>8}")
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
