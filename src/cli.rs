use crate::file_sizes::{GIGABYTE, KILOBYTE, MEGABYTE};
use clap::Parser;

const SIZE_ALIASES: &[(&str, usize)] = &[
    ("blank", 0),
    ("config", 10),
    ("code", 100 * KILOBYTE),
    ("excel", MEGABYTE),
    ("document", 5 * MEGABYTE),
    ("image", 10 * MEGABYTE),
    ("gif", 20 * MEGABYTE),
    ("audio", 50 * MEGABYTE),
    ("video", 500 * MEGABYTE),
    ("large", GIGABYTE),
];

fn get_size_by_alias(alias: &str) -> Option<usize> {
    SIZE_ALIASES
        .iter()
        .find_map(|&(key, size)| if key == alias { Some(size) } else { None })
}

#[derive(Parser, Debug)]
#[command(name = "Main command")]
#[command(version = "0.0.1")]
#[command(about = "Traverses a directory and processes files based on extensions")]
pub struct MainCommandArgs {
    /// The directory to traverse
    pub directory: String,

    /// List of file extensions to consider
    #[arg(short = 'e', long = "extensions", default_value = "")]
    pub file_extensions: String,

    /// Minimum file size to consider (alias)
    #[arg(long = "min_file_size", default_value = "code")]
    pub min_file_size: String,

    /// Number of top files to return based on size
    #[arg(short = 'n', long = "n_top", default_value = "5")]
    pub n_top: usize,
}

pub struct CLIArgs {
    pub directory: String,
    pub file_extensions: Vec<String>,
    pub min_file_size: usize,
    pub n_top: usize,
}

pub fn parse_args() -> CLIArgs {
    let cli_args = MainCommandArgs::parse();

    // Handle file_extensions parsing
    let file_extensions: Vec<String> = if cli_args.file_extensions.is_empty() {
        Vec::new()
    } else {
        cli_args
            .file_extensions
            .split(',')
            .map(String::from)
            .collect()
    };

    let size_alias = &cli_args.min_file_size;
    let min_file_size = get_size_by_alias(size_alias).unwrap_or(100 * KILOBYTE);

    CLIArgs {
        directory: cli_args.directory,
        file_extensions,
        min_file_size,
        n_top: cli_args.n_top,
    }
}
