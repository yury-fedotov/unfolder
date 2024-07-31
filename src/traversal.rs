use crate::file_utils::FileInfo;
use crate::file_utils::{calculate_hash, get_file_size};
use ignore::WalkBuilder;

pub struct DirectoryTraversalOutput {
    pub file_infos: Vec<FileInfo>,
    pub dir_count: usize,
    pub max_depth: usize,
}

pub fn traverse_directory(dir: &str, min_file_size: &usize) -> DirectoryTraversalOutput {
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
                let hash = if size > *min_file_size as u64 {
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
