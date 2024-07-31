use crate::file_utils::FileInfo;
use crate::traversal::CompleteTraversalStatistics;

use colored::*;
use num_format::{Locale, ToFormattedString};

/// Struct to hold the results of the file analysis
pub struct AnalysisResults {
    pub elapsed_time: std::time::Duration,
    pub complete_statistics: CompleteTraversalStatistics,
    pub largest_files: Vec<FileInfo>,
}

impl AnalysisResults {
    /// Prints the results of the file analysis
    pub fn print_results(&self) {
        println!();
        println!("{}", "Run overview:".bold().underline().yellow());

        // Convert elapsed time to milliseconds and round to nearest integer
        let elapsed_ms = (self.elapsed_time.as_millis() as f64).round();

        println!("{}", format!("Elapsed time: {} ms", elapsed_ms).blue());

        // Format file_count and dir_count with a thousand separators
        let file_count_formatted = self
            .complete_statistics
            .n_files_analyzed
            .to_formatted_string(&Locale::en);
        let dir_count_formatted = self
            .complete_statistics
            .n_directories_visited
            .to_formatted_string(&Locale::en);

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
            format!(
                "Deepest level of folder nesting: {}",
                self.complete_statistics.max_depth_visited
            )
            .green()
        );

        println!();
        println!("{}", "Largest files:".bold().underline().yellow());

        for file in &self.largest_files {
            println!(
                "{}: {}",
                file.path.display().to_string().cyan(),
                format_size(file.size).magenta()
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
