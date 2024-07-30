use std::fs;
use std::path::PathBuf;

pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub hash: String,
}

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

pub fn find_largest_files(files: Vec<FileInfo>, count: usize) -> Vec<FileInfo> {
    let mut sorted_files: Vec<FileInfo> = files.into_iter().collect();
    sorted_files.sort_by(|a, b| b.size.cmp(&a.size));
    sorted_files.into_iter().take(count).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

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
}
