mod file_utils;
mod results;
mod traversal;

use clap::Parser;
use file_utils::{find_duplicate_groups, get_largest_files};
use results::AnalysisResults;
use std::time::Instant;
use traversal::traverse_directory;

/// Program to find and print the largest files in a directory and its subdirectories
#[derive(Parser)]
struct Args {
    /// The directory to search
    directory: String,

    /// The number of largest files to find
    #[clap(short, long, default_value_t = 5)]
    count: usize,
}

fn main() {
    let start_time = Instant::now();

    let args = Args::parse();
    let dir = &args.directory;
    let count = args.count;

    let traverval_output = traverse_directory(dir);
    let file_count = traverval_output.file_infos.len();
    let largest_files = get_largest_files(&traverval_output.file_infos, count);
    let dir_count = traverval_output.dir_count;
    let max_depth = traverval_output.max_depth;

    let elapsed_time = start_time.elapsed();

    // Create the AnalysisResults struct
    let results = AnalysisResults {
        elapsed_time,
        file_count,
        dir_count,
        max_depth,
        largest_files,
    };

    // Call the print_results method on the results instance
    results.print_results();

    find_duplicate_groups(&traverval_output.file_infos);
}
