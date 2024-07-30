use clap::Parser;

/// Program to find and print the largest files in a directory and its subdirectories
#[derive(Parser)]
pub struct Args {
    /// The directory to search
    pub directory: String,

    /// The number of largest files to find
    #[clap(short, long, default_value_t = 5)]
    pub n_top: usize,
}
