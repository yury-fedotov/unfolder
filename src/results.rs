use crate::file_utils::FileInfo;
use crate::traversal::CompleteTraversalStatistics;

use crate::file_sizes::format_size;
use colored::*;
use num_format::{Locale, ToFormattedString};

/// Struct to hold the results of the file analysis
pub struct AnalysisResults {
    pub elapsed_time: std::time::Duration,
    pub complete_statistics: CompleteTraversalStatistics,
    pub largest_files: Vec<FileInfo>,
    pub duplicate_groups: Vec<(String, Vec<FileInfo>)>,
}

impl AnalysisResults {
    /// Prints the results of the file analysis
    pub fn print_results(&self) {
        println!();
        println!("{}", "Run overview:".bold().underline().yellow());

        // Convert elapsed time to milliseconds and round to nearest integer
        let elapsed_ms = (self.elapsed_time.as_millis() as f64).round();

        println!("{}", format!("⏱️ Elapsed time: {} ms", elapsed_ms).blue());

        // Format file_count and dir_count with a thousand separators
        let n_files_identified_formatted = self
            .complete_statistics
            .n_files_identified
            .to_formatted_string(&Locale::en);
        let dir_count_formatted = self
            .complete_statistics
            .n_directories_visited
            .to_formatted_string(&Locale::en);
        let n_files_considered_formatted = self
            .complete_statistics
            .n_files_considered
            .to_formatted_string(&Locale::en);
        let n_files_hashed_formatted = self
            .complete_statistics
            .n_files_hashed
            .to_formatted_string(&Locale::en);

        println!(
            "{}",
            format!(
                "📂 Directories: {} traversed, {} levels of nesting",
                dir_count_formatted, self.complete_statistics.max_depth_visited,
            )
            .green()
        );
        println!(
            "{}",
            format!(
                "📄 Files: {} identified, {} of relevant types, {} are analyzed for content",
                n_files_identified_formatted,
                n_files_considered_formatted,
                n_files_hashed_formatted
            )
            .green()
        );

        println!();
        println!("{}", "Largest files:".bold().underline().yellow());

        for file in &self.largest_files {
            println!(
                "{}: {}",
                file.path.display().to_string().cyan(),
                format_size(file.size as usize).magenta()
            );
        }

        println!();
        println!("{}", "Duplicated files:".bold().underline().yellow());

        println!();
        for (hash, group) in &self.duplicate_groups {
            println!("Hash: {}", hash);
            for file in group {
                println!("{} (size: {})", file.path.display(), format_size(file.size as usize));
            }
            println!();
        }
    }
}
