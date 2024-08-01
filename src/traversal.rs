use crate::file_utils;
use crate::file_utils::FileInfo;
use crate::file_utils::{calculate_hash, get_file_size};
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct DirectoryTraversalOutput {
    pub complete_statistics: CompleteTraversalStatistics,
    pub file_infos: Vec<FileInfo>,
}

pub struct CompleteTraversalStatistics {
    pub n_files_analyzed: usize,
    pub n_directories_visited: usize,
    pub max_depth_visited: usize,
}

pub fn traverse_directory(
    dir: &str,
    min_file_size: &usize,
    file_extensions: &[String],
) -> DirectoryTraversalOutput {
    let n_directories_visited = Arc::new(AtomicUsize::new(1));
    let max_depth_visited = Arc::new(AtomicUsize::new(0));
    let n_files_analyzed = Arc::new(AtomicUsize::new(0));

    let entries: Vec<_> = WalkBuilder::new(dir)
        .hidden(true)
        .ignore(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build()
        .filter_map(|e| e.ok())
        .collect();

    let max_depth_visited_clone = Arc::clone(&max_depth_visited);
    let n_directories_visited_clone = Arc::clone(&n_directories_visited);
    let n_files_analyzed_clone = Arc::clone(&n_files_analyzed);

    let file_infos: Vec<FileInfo> = entries
        .into_par_iter() // Use Rayon parallel iterator
        .filter_map(|entry| {
            let depth = entry.depth();
            if depth > max_depth_visited_clone.load(Ordering::Relaxed) {
                max_depth_visited_clone.store(depth, Ordering::Relaxed);
            }
            if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                n_directories_visited_clone.fetch_add(1, Ordering::Relaxed);
                return None;
            }
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                n_files_analyzed_clone.fetch_add(1, Ordering::Relaxed);
                let path = entry.into_path();
                if !file_extensions.is_empty()
                    && !file_utils::has_allowed_extension(&path, file_extensions)
                {
                    return None;
                }
                let size = get_file_size(&path);
                let hash = if size > *min_file_size as u64 {
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
        n_files_analyzed: n_files_analyzed.load(Ordering::Relaxed),
        n_directories_visited: n_directories_visited.load(Ordering::Relaxed),
        max_depth_visited: max_depth_visited.load(Ordering::Relaxed),
    };

    DirectoryTraversalOutput {
        complete_statistics,
        file_infos,
    }
}
