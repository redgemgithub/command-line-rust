use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Config {
    /// Input file(s)
    #[arg(default_value("-"), value_name("FILE"))]
    files: Vec<String>,

    /// Number of lines
    #[arg(short('n'), long, default_value("10"), value_parser(1..))]
    lines: i64,

    /// Number of bytes
    #[arg(short('c'), long, conflicts_with("lines"), value_parser(clap::value_parser!(u64).range(1..)))]
    bytes: Option<u64>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let args = Config::parse();
    Ok(Config {
        files: args.files,
        lines: args.lines,
        bytes: args.bytes,
    })
    // let matches = App::new("headr")
    //     .version("0.1.0")
    //     .author("Ken Youens-Clark <kyclark@gmail.com>")
    //     .about("Rust head")
    //     .arg(
    //         Arg::with_name("lines")
    //             .short("n")
    //             .long("lines")
    //             .value_name("LINES")
    //             .help("Number of lines")
    //             .default_value("10"),
    //     )
    //     .arg(
    //         Arg::with_name("bytes")
    //             .short("c")
    //             .long("bytes")
    //             .value_name("BYTES")
    //             .takes_value(true)
    //             .conflicts_with("lines")
    //             .help("Number of bytes"),
    //     )
    //     .arg(
    //         Arg::with_name("files")
    //             .value_name("FILE")
    //             .help("Input file(s)")
    //             .multiple(true)
    //             .default_value("-"),
    //     )
    //     .get_matches();

    // let lines = matches
    //     .value_of("lines")
    //     .map(parse_positive_int)
    //     .transpose()
    //     .map_err(|e| format!("illegal line count -- {}", e))?;

    // let bytes = matches
    //     .value_of("bytes")
    //     .map(parse_positive_int)
    //     .transpose()
    //     .map_err(|e| format!("illegal byte count -- {}", e))?;

    // Ok(Config {
    //     files: matches.values_of_lossy("files").unwrap(),
    //     lines: lines.unwrap(),
    //     bytes,
    // })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {} <==", if file_num > 0 { "\n" } else { "" }, &filename);
                }

                match config.bytes {
                    Some(num_bytes) => {
                        let mut handle = file.take(num_bytes);
                        let mut buffer = vec![0; num_bytes as usize];
                        let bytes_read = handle.read(&mut buffer)?;
                        print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    },
                    None => {
                        for _ in 0..config.lines {
                            let mut line = String::new();
                            match file.read_line(&mut line) {
                                Ok(0) => break,
                                Ok(_) => {
                                    print!("{}", line);
                                    line.clear();
                                },
                                Err(e) => return Err(e)?,
                            }
                        }
                    },
                }
                // if let Some(num_bytes) = config.bytes {
                //     let mut handle = file.take(num_bytes as u64);
                //     let mut buffer = vec![0; num_bytes as usize];
                //     let bytes_read = handle.read(&mut buffer)?;
                //     print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                // } else {
                //     let mut line = String::new();
                //     for _ in 0..config.lines {
                //         let bytes = file.read_line(&mut line)?;
                //         if bytes == 0 {
                //             break;
                //         }
                //         print!("{}", line);
                //         line.clear();
                //     }
                // }
            },
            Err(err) => eprintln!("{}: {}", filename, err),
        }
    }
    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// --------------------------------------------------
#[cfg(test)]
fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

// --------------------------------------------------
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
