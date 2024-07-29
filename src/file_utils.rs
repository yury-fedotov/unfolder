use std::fs;
use std::path::PathBuf;

pub fn get_file_size(path: &PathBuf) -> u64 {
    fs::metadata(path)
        .map(|metadata| metadata.len())
        .unwrap_or(0)
}

pub fn find_largest_files(files: Vec<PathBuf>, count: usize) -> Vec<(PathBuf, u64)> {
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
