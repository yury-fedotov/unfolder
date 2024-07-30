use clap::{Arg, Command};

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
                .help("Minimum file size to consider")
                .long("min_file_size")
                .default_value("3145728"),
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
    let min_file_size = matches
        .get_one::<String>("min_file_size")
        .map(|s| s.parse().unwrap_or(3145728))
        .unwrap_or(3145728);
    let n_top = matches
        .get_one::<String>("n_top")
        .map(|s| s.parse().unwrap_or(5)) // Parse the value and default to 5 on error
        .unwrap_or(5);

    CLIArgs {
        directory,
        min_file_size,
        n_top,
    }
}
