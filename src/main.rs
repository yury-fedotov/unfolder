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

    let traversal_output = traverse_directory(&args.directory, &args.min_file_size);
    let largest_files = get_largest_files(&traversal_output.file_infos, &args.n_top);

    let elapsed_time = start_time.elapsed();

    // Create the AnalysisResults struct
    let results = AnalysisResults {
        elapsed_time,
        complete_statistics: traversal_output.complete_statistics,
        largest_files,
    };

    // Call the print_results method on the results instance
    results.print_results();

    find_duplicate_groups(&traversal_output.file_infos);
}
