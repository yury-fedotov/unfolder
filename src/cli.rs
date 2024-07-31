use clap::{Arg, Command};

const THOUSAND: usize = 1000;
const MEGA_THOUSAND_EXPONENT: u32 = 2;
const GIGA_THOUSAND_EXPONENT: u32 = 3;
const KILOBYTE: usize = THOUSAND;
const MEGABYTE: usize = THOUSAND.pow(MEGA_THOUSAND_EXPONENT);
const GIGABYTE: usize = THOUSAND.pow(GIGA_THOUSAND_EXPONENT);

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

    let min_file_size = get_size_by_alias(size_alias.as_str()).unwrap_or(MEGABYTE); // Default to 1 MB if alias not found

    CLIArgs {
        directory,
        min_file_size,
        n_top,
    }
}
