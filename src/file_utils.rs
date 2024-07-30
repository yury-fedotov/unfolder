use std::fs;
use std::path::PathBuf;

/// Retrieves the size of a file.
///
/// This function takes a path to a file and returns its size in bytes. If the file does not
/// exist or an error occurs while accessing the file metadata, it returns 0.
///
/// # Arguments
///
/// * `path` - A `PathBuf` representing the path to the file.
///
/// # Returns
///
/// Returns a `u64` representing the size of the file in bytes. If the file cannot be accessed or
/// does not exist, it returns 0.
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_get_file_size() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");

        // Create a temporary file with known content
        let content = b"Hello, world!";
        fs::write(&file_path, content).unwrap();

        // Verify the size of the file
        let path = PathBuf::from(file_path);
        let size = get_file_size(&path);
        assert_eq!(size, content.len() as u64);

        // Verify the function handles non-existent files gracefully
        let non_existent_path = PathBuf::from(dir.path().join("non_existent_file.txt"));
        let size = get_file_size(&non_existent_path);
        assert_eq!(size, 0);
    }

    #[test]
    fn test_find_largest_files() {
        let dir = tempdir().unwrap();

        // Create temporary files with known sizes
        let file1 = NamedTempFile::new_in(&dir).unwrap();
        fs::write(file1.path(), b"small").unwrap(); // 5 bytes

        let file2 = NamedTempFile::new_in(&dir).unwrap();
        fs::write(file2.path(), b"medium_size").unwrap(); // 11 bytes

        let file3 = NamedTempFile::new_in(&dir).unwrap();
        fs::write(file3.path(), b"much_larger_file_content").unwrap(); // 24 bytes

        // Collect file paths
        let files = vec![
            file1.path().to_path_buf(),
            file2.path().to_path_buf(),
            file3.path().to_path_buf(),
        ];

        // Find the largest files
        let largest_files = find_largest_files(files, 2);

        // Print debug information
        for (file, size) in &largest_files {
            println!("File: {:?}, Size: {}", file.display(), size);
        }

        // Expected results
        let expected_files = vec![
            (file3.path().to_path_buf(), 24),
            (file2.path().to_path_buf(), 11),
        ];

        // Check if the results match expected
        assert_eq!(largest_files.len(), 2);
        assert_eq!(largest_files, expected_files);
    }
}
