mod cli;
mod file_sizes;
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
    let duplicate_groups = find_duplicate_groups(&traversal_output.file_infos);

    let elapsed_time = start_time.elapsed();

    let results = AnalysisResults {
        elapsed_time,
        complete_statistics: traversal_output.complete_statistics,
        largest_files,
        duplicate_groups,
    };

    results.print_results();
}
