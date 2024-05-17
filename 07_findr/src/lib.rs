use clap::{builder::PossibleValue, Parser, ValueEnum};
use anyhow::Result;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }
    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Config {
    /// Search path(s)
    #[arg(value_name("PATH"), default_value("."))]
    paths: Vec<String>,

    /// Name
    #[arg(short('n'), long("name"), value_name("NAME"), value_parser(Regex::new), num_args(0..))]
    names: Vec<Regex>,

    /// Entry type
    #[arg(short('t'), long("type"), value_name("TYPE"), value_parser(clap::value_parser!(EntryType)), num_args(0..))]
    entry_types: Vec<EntryType>,
}

// --------------------------------------------------
pub fn get_args() -> Result<Config> {
    Ok(Config::parse())
}

// --------------------------------------------------
pub fn run(config: Config) -> Result<()> {
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty()
        || config.entry_types.iter()
            .any(|entry_type| match entry_type {
                EntryType::Link => entry.file_type().is_symlink(),
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
            })
    };

    let name_filter = |entry: &DirEntry| {
        config.names.is_empty()
        || config.names.iter()
            .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in &config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }

    Ok(())
}
