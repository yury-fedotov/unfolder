use colored::*;
use num_format::{Locale, ToFormattedString};
use std::path::PathBuf;

/// Struct to hold the results of the file analysis
pub struct AnalysisResults {
    pub elapsed_time: std::time::Duration,
    pub file_count: usize,
    pub dir_count: usize,
    pub max_depth: usize,
    pub largest_files: Vec<(PathBuf, u64)>,
}

impl AnalysisResults {
    /// Prints the results of the file analysis
    pub fn print_results(&self) {
        println!();
        println!("{}", "Run overview:".bold().underline().yellow());

        // Convert elapsed time to milliseconds and round to nearest integer
        let elapsed_ms = (self.elapsed_time.as_millis() as f64).round();

        println!("{}", format!("Elapsed time: {} ms", elapsed_ms).blue());

        // Format file_count and dir_count with thousand separators
        let file_count_formatted = self.file_count.to_formatted_string(&Locale::en);
        let dir_count_formatted = self.dir_count.to_formatted_string(&Locale::en);

        println!(
            "{}",
            format!("Number of files analyzed: {}", file_count_formatted).green()
        );
        println!(
            "{}",
            format!("Number of directories traversed: {}", dir_count_formatted).green()
        );
        println!(
            "{}",
            format!("Deepest level of folder nesting: {}", self.max_depth).green()
        );

        println!();
        println!("{}", "Largest files:".bold().underline().yellow());

        for (file, size) in &self.largest_files {
            println!(
                "{}: {}",
                file.display().to_string().cyan(),
                format_size(*size).magenta()
            );
        }
    }
}

/// Converts bytes into a human-readable format (bytes, KB, MB, GB)
fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} bytes", size)
    }
}
