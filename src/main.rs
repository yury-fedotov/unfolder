mod cli;
mod file_utils;
mod results;
mod traversal;

use cli::parse_args;
use file_utils::{find_duplicate_groups, get_largest_files};
use results::AnalysisResults;
use std::time::Instant;
use traversal::traverse_directory;

fn main() {
    let start_time = Instant::now();

    let args = parse_args();

    let traverval_output = traverse_directory(&args.directory);
    let file_count = traverval_output.file_infos.len();
    let largest_files = get_largest_files(&traverval_output.file_infos, &args.n_top);
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
