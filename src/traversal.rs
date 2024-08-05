use crate::file_utils;
use crate::file_utils::FileInfo;
use crate::file_utils::{calculate_hash, get_file_size};
use ignore::WalkBuilder;
use rayon::prelude::*;

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
    let entries: Vec<_> = WalkBuilder::new(dir)
        .standard_filters(true)
        .build()
        .filter_map(|e| e.ok())
        .collect();

    let n_directories_visited = entries
        .par_iter()
        .filter(|e| e.file_type().map_or(false, |ft| ft.is_dir()))
        .count();
    let max_depth_visited = entries.par_iter().map(|e| e.depth()).max().unwrap_or(0);
    let n_files_identified = entries
        .par_iter()
        .filter(|e| e.file_type().map_or(false, |ft| ft.is_file()))
        .count();

    let file_infos: Vec<FileInfo> = entries
        .into_par_iter()
        .filter_map(|entry| {
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                let path = entry.into_path();
                if !file_extensions.is_empty()
                    && !file_utils::has_allowed_extension(&path, file_extensions)
                {
                    return None;
                }
                let size = get_file_size(&path);
                let should_calculate_hash = size > *min_file_size as u64;
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

    let n_files_considered = file_infos.len();
    let n_files_hashed = file_infos
        .iter()
        .filter(|file_info| !file_info.hash.is_empty())
        .count();

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
