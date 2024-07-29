use clap::Parser;
use colored::*;
use ignore::WalkBuilder;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

/// Program to find and print the largest files in a directory and its subdirectories
#[derive(Parser)]
struct Args {
    /// The directory to search
    directory: String,

    /// The number of largest files to find
    #[clap(short, long, default_value_t = 5)]
    count: usize,
}

fn get_files_in_dir(dir: &str) -> (Vec<PathBuf>, usize, usize) {
    let mut dir_count = 1;
    let mut max_depth = 0;
    let files: Vec<PathBuf> = WalkBuilder::new(dir)
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
                Some(entry.into_path())
            } else {
                None
            }
        })
        .collect();

    (files, dir_count, max_depth)
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

    let (files, dir_count, max_depth) = get_files_in_dir(dir);
    let file_count = files.len();
    let largest_files = find_largest_files(files, count);

    let elapsed_time = start_time.elapsed();

    println!();
    println!("{}", "Run overview:".bold().underline().yellow());

    println!("{}", format!("Elapsed time: {:?}", elapsed_time).blue());
    println!(
        "{}",
        format!("Number of files analyzed: {}", file_count).green()
    );
    println!(
        "{}",
        format!("Number of directories traversed: {}", dir_count).green()
    );
    println!(
        "{}",
        format!("Deepest level of folder nesting: {}", max_depth).green()
    );

    println!();
    println!("{}", "Largest files:".bold().underline().yellow());

    for (file, size) in largest_files {
        println!(
            "{}: {} bytes",
            file.display().to_string().cyan(),
            size.to_string().magenta()
        );
    }
}
