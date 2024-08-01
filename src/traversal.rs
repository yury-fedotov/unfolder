use crate::file_utils;
use crate::file_utils::FileInfo;
use crate::file_utils::{calculate_hash, get_file_size};
use ignore::WalkBuilder;

pub struct DirectoryTraversalOutput {
    pub complete_statistics: CompleteTraversalStatistics,
    pub file_infos: Vec<FileInfo>,
}

pub struct CompleteTraversalStatistics {
    pub n_files_identified: usize,
    pub n_files_considered: usize,
    pub n_files_hashed: usize,
    pub n_directories_visited: usize,
    pub max_depth_visited: usize,
}

pub fn traverse_directory(
    dir: &str,
    min_file_size: &usize,
    file_extensions: &[String],
) -> DirectoryTraversalOutput {
    let mut n_directories_visited = 1;
    let mut max_depth_visited = 0;
    let mut n_files_identified = 0;
    let mut n_files_considered = 0;
    let mut n_files_hashed = 0;
    let file_infos: Vec<FileInfo> = WalkBuilder::new(dir)
        .standard_filters(true)
        .build()
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            let depth = entry.depth();
            if depth > max_depth_visited {
                max_depth_visited = depth;
            }
            if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                n_directories_visited += 1;
            }
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                n_files_identified += 1;
                let path = entry.into_path();
                if !file_extensions.is_empty()
                    && !file_utils::has_allowed_extension(&path, file_extensions)
                {
                    return None;
                }
                n_files_considered += 1;
                let size = get_file_size(&path);
                let should_calculate_hash = size > *min_file_size as u64;
                if should_calculate_hash {
                    n_files_hashed += 1;
                }
                let hash = if should_calculate_hash {
                    calculate_hash(&path).unwrap_or_else(|_| String::new())
                } else {
                    String::new()
                };
                Some(FileInfo { path, size, hash })
            } else {
                None
            }
        })
        .collect();

    let complete_statistics = CompleteTraversalStatistics {
        n_files_identified,
        n_files_considered,
        n_files_hashed,
        n_directories_visited,
        max_depth_visited,
    };
    DirectoryTraversalOutput {
        complete_statistics,
        file_infos,
    }
}
