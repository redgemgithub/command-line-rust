use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use anyhow::Result as MyResult;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Config {
    /// Input file(s)
    #[arg(value_name("FILE"), default_value("-"))]
    files: Vec<String>,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show byte count
    #[arg(short('c'), long)]
    bytes: bool,

    /// Show character count
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    filename: String,
    error_msg: String,
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let mut args = Config::parse();
    if [args.lines, args.words, args.bytes, args.chars].iter().all(|v| v == &false) {
        args = Config { lines: true, words: true, bytes: true, ..args };
    }
    Ok(Config {..args})

}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    // 全ファイルを通して桁数を決定する
    let mut filesinfo = Vec::<FileInfo>::new();
    for filename in &config.files {
        match open(filename) {
            Err(err) => filesinfo.push(FileInfo{
                filename: filename.to_string(),
                error_msg: format!("{}: {}", filename, err),
                num_lines: 0,
                num_words: 0,
                num_bytes: 0,
                num_chars: 0,
            }), // eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(filename.clone(), file) {
                    filesinfo.push(FileInfo{
                        filename: filename.clone(),
                        error_msg: "".to_string(),
                        num_lines: info.num_lines,
                        num_words: info.num_words,
                        num_bytes: info.num_bytes,
                        num_chars: info.num_chars,
                    });
                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }
    // 表示する中で最大の桁数を取得
    let fixed_digits = config.files.contains(&"-".to_string());
    let digits = match fixed_digits {
        // 標準入力を含む場合は7桁固定
        true => 7,
        // そうでなければ表示する項目の中での最大桁
        false => {
            let mut digits = Vec::<usize>::new();
            if config.lines {
                digits.push(total_lines);
            }
            if config.words {
                digits.push(total_words);
            }
            if config.bytes {
                digits.push(total_bytes);
            }
            if config.chars {
                digits.push(total_chars);
            }
            digits.iter().max().unwrap_or(&0).to_string().len()
        },
    };
    let digits = match [config.lines, config.words, config.bytes, config.chars].iter().filter(|flag| **flag).count() {
        // 表示項目が1つなら最小桁は1桁
        1 => std::cmp::max(1, digits),
        // 表示項目が複数でもすべて0なら最小桁は1桁
        _ if total_lines + total_words + total_bytes + total_chars == 0 => 1,
        // それ以外の最小桁は2桁
        _ => std::cmp::max(2, digits),
    };

    for info in filesinfo {
        if info.error_msg != "" {
            eprintln!("{}: {}", info.filename, info.error_msg);
            continue;
        }
        println!(
            "{}{}{}{}{}",
            format_field(info.num_lines, digits, config.lines),
            format_field(info.num_words, digits, config.words),
            format_field(info.num_bytes, digits, config.bytes),
            format_field(info.num_chars, digits, config.chars),
            if info.filename == "-" {
                "".to_string()
            } else {
                format!("{}", &info.filename)
            },
        );
    }
    if config.files.len() > 1 {
        println!(
            "{}{}{}{}total",
            format_field(total_lines, digits, config.lines),
            format_field(total_words, digits, config.words),
            format_field(total_bytes, digits, config.bytes),
            format_field(total_chars, digits, config.chars)
        );
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
fn format_field(value: usize, digits: usize, show: bool) -> String {
    match show {
        true => format!("{:>digits$} ", value),
        false => "".to_string(),
    }
}

// --------------------------------------------------
pub fn count(filename: String, mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        filename,
        error_msg: "".to_string(),
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let filename = "".to_string();
        let info = count(filename.clone(), Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            filename: filename.clone(),
            error_msg: "".to_string(),
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, 1, false), "");
        assert_eq!(format_field(3, 1, true), "3 ");
        assert_eq!(format_field(10, 2, true), "10 ");
    }
}
