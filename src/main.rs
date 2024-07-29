use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

/// Program to find and print the largest files in a directory and its subdirectories
#[derive(Parser)]
struct Args {
    /// The directory to search
    directory: String,

    /// The number of largest files to find
    #[clap(short, long, default_value_t = 5)]
    count: usize,
}

fn get_files_in_dir(dir: &str) -> (Vec<PathBuf>, usize) {
    let mut dir_count = 1;
    let files: Vec<PathBuf> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            if entry.file_type().is_dir() {
                dir_count += 1;
            }
            if entry.file_type().is_file() {
                Some(entry.into_path())
            } else {
                None
            }
        })
        .collect();

    (files, dir_count)
}

fn get_file_size(path: &PathBuf) -> u64 {
    fs::metadata(path)
        .map(|metadata| metadata.len())
        .unwrap_or(0)
}

fn find_largest_files(files: Vec<PathBuf>, count: usize) -> Vec<(PathBuf, u64)> {
    let mut files_with_sizes: Vec<(PathBuf, u64)> = files
        .into_iter()
        .map(|file| {
            let size = get_file_size(&file);
            (file, size)
        })
        .collect();

    files_with_sizes.sort_by(|a, b| b.1.cmp(&a.1));
    files_with_sizes.into_iter().take(count).collect()
}

fn main() {
    let start_time = Instant::now();

    let args = Args::parse();
    let dir = &args.directory;
    let count = args.count;

    let (files, dir_count) = get_files_in_dir(dir);
    let file_count = files.len();
    let largest_files = find_largest_files(files, count);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    println!("Number of files analyzed: {}", file_count);
    println!("Number of directories traversed: {}", dir_count);
    println!();

    for (file, size) in largest_files {
        println!("{:?}: {} bytes", file, size);
    }
}
