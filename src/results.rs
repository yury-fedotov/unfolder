use crate::file_utils::FileInfo;
use crate::output_format::{print_file_path_with_size, OutputFormat};
use crate::traversal::CompleteTraversalStatistics;
use colored::*;
use num_format::{Locale, ToFormattedString};

pub struct AnalysisResults {
    pub elapsed_time: std::time::Duration,
    pub complete_statistics: CompleteTraversalStatistics,
    pub largest_files: Vec<FileInfo>,
    pub duplicate_groups: Vec<(String, Vec<FileInfo>)>,
}

impl AnalysisResults {
    pub fn print_results(&self) {
        println!();
        println!(
            "{}",
            "Run overview:"
                .bold()
                .underline()
                .color(OutputFormat::Headers.color())
        );

        // Convert elapsed time to milliseconds and round to nearest integer
        let elapsed_ms = (self.elapsed_time.as_millis() as f64).round();

        println!(
            "{} {}",
            "⏱️ Elapsed time:".to_string().bold(),
            format!("{} ms", elapsed_ms)
                .bold()
                .color(OutputFormat::Numbers.color())
        );

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
            "{} {} traversed, {} levels of nesting",
            "📂 Directories:".to_string().bold(),
            dir_count_formatted
                .to_string()
                .bold()
                .color(OutputFormat::Numbers.color()),
            format!("{}", self.complete_statistics.max_depth_visited)
                .bold()
                .color(OutputFormat::Numbers.color()),
        );
        println!(
            "{} {} identified, {} of relevant types, {} analyzed for content",
            "📄 Files:".to_string().bold(),
            n_files_identified_formatted
                .to_string()
                .bold()
                .color(OutputFormat::Numbers.color()),
            n_files_considered_formatted
                .to_string()
                .bold()
                .color(OutputFormat::Numbers.color()),
            n_files_hashed_formatted
                .to_string()
                .bold()
                .color(OutputFormat::Numbers.color()),
        );

        println!();
        println!(
            "{}",
            "Largest files:"
                .bold()
                .underline()
                .color(OutputFormat::Headers.color())
        );

        for file in &self.largest_files {
            print_file_path_with_size(file)
        }

        println!();
        println!(
            "{}",
            "Duplicated files:"
                .bold()
                .underline()
                .color(OutputFormat::Headers.color())
        );

        if !self.duplicate_groups.is_empty() {
            for (index, (_hash, group)) in self.duplicate_groups.iter().enumerate() {
                println!("Group {}:", index + 1);
                for file in group {
                    print_file_path_with_size(file)
                }
            }
        } else {
            println!("None found!");
        }
    }
}
