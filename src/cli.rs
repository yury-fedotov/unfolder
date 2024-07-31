use clap::{Arg, Command};

const SIZE_ALIASES: &[(&str, usize)] = &[
    ("blank", 1024),               // 1 KB
    ("config", 10 * 1024),         // 10 KB
    ("code", 100 * 1024),          // 100 KB
    ("excel", 1024 * 1024),        // 1 MB
    ("document", 5 * 1024 * 1024), // 5 MB
    ("image", 10 * 1024 * 1024),   // 10 MB
    ("gif", 20 * 1024 * 1024),     // 20 MB
    ("audio", 50 * 1024 * 1024),   // 50 MB
    ("video", 500 * 1024 * 1024),  // 500 MB
    ("large", 1024 * 1024 * 1024), // 1 GB
];

fn get_size_by_alias(alias: &str) -> Option<usize> {
    SIZE_ALIASES
        .iter()
        .find_map(|&(key, size)| if key == alias { Some(size) } else { None })
}

pub struct CLIArgs {
    pub directory: String,
    pub min_file_size: usize,
    pub n_top: usize,
}

pub fn parse_args() -> CLIArgs {
    let matches = Command::new("Directory Traversal")
        .version("1.0")
        .long_about("Traverses a directory and processes files based on extensions")
        .arg(
            Arg::new("directory")
                .help("The directory to traverse")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("min_file_size")
                .help("Minimum file size to consider (alias)")
                .long("min_file_size")
                .default_value("document"),
        )
        .arg(
            Arg::new("n_top")
                .help("Number of top files to return based on size")
                .short('n')
                .long("n_top")
                .default_value("5"), // Default to 5
        )
        .get_matches();

    let directory = matches
        .get_one::<String>("directory")
        .expect("Directory argument missing")
        .clone();
    let size_alias = matches
        .get_one::<String>("min_file_size")
        .expect("Size argument missing");
    let n_top = matches
        .get_one::<String>("n_top")
        .map(|s| s.parse().unwrap_or(5)) // Parse the value and default to 5 on error
        .unwrap_or(5);

    let min_file_size = get_size_by_alias(size_alias.as_str()).unwrap_or(1024 * 1024 * 1024); // Default to 1 GB if alias not found

    CLIArgs {
        directory,
        min_file_size,
        n_top,
    }
}
