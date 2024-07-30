mod file_utils;
mod results;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Read};

use clap::Parser;
use file_utils::{find_duplicate_groups, get_file_size, get_largest_files, FileInfo};
use ignore::WalkBuilder;
use results::{AnalysisResults, DirectoryTraversalOutput};
use std::path::PathBuf;
use std::time::Instant;

fn calculate_hash(path: &PathBuf) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

/// Program to find and print the largest files in a directory and its subdirectories
#[derive(Parser)]
struct Args {
    /// The directory to search
    directory: String,

    /// The number of largest files to find
    #[clap(short, long, default_value_t = 5)]
    count: usize,
}

fn traverse_directory(dir: &str) -> DirectoryTraversalOutput {
    let mut dir_count = 1;
    let mut max_depth = 0;
    let file_infos: Vec<FileInfo> = WalkBuilder::new(dir)
        .hidden(true)
        .ignore(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build()
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            let depth = entry.depth();
            if depth > max_depth {
                max_depth = depth;
            }
            if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                dir_count += 1;
            }
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                let path = entry.into_path();
                let size = get_file_size(&path);
                let hash = if size > 3 * 1024 * 1024 {
                    // 3MB in bytes
                    match calculate_hash(&path) {
                        Ok(hash) => hash,
                        Err(_) => String::new(),
                    }
                } else {
                    String::new()
                };
                Some(FileInfo { path, size, hash })
            } else {
                None
            }
        })
        .collect();

    DirectoryTraversalOutput {
        file_infos,
        dir_count,
        max_depth,
    }
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
