use clap::Parser;
use anyhow::{Result, anyhow};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Config {
    /// Input file
    #[arg(value_name("IN_FILE"), default_value("-"))]
    in_file: String,

    /// Output file
    #[arg(value_name("OUT_FILE"))]
    out_file: Option<String>,

    /// Show counts
    #[arg(short('c'), long("count"))]
    count: bool,
}

// --------------------------------------------------
pub fn get_args() -> Result<Config> {
    Ok(Config::parse())
}

// --------------------------------------------------
pub fn run(config: Config) -> Result<()> {
    let mut in_file = open(&config.in_file)
        .map_err(|e| anyhow!("{}: {}", config.in_file, e))?;

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> Result<()> {
        if count > 0 {
            let count_digits = match config.count {
                true => format!("{:>4} ", count),
                false => format!(""),
            };
            write!(out_file, "{}{}", count_digits, text)?;
        };
        Ok(())
    };

    let mut line = String::new();
    let mut prev = String::new();
    let mut count: u64 = 0;
    loop {
        line.clear();
        let bytes = in_file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != prev.trim_end() {
            print(count, &prev)?;
            prev = line.clone();
            count = 0;
        }
        count += 1;
    }
    print(count, &prev)?;

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
