use crate::file_utils::FileInfo;
use crate::traversal::CompleteTraversalStatistics;

use crate::file_sizes::format_size;
use colored::*;
use num_format::{Locale, ToFormattedString};

pub struct AnalysisResults {
    pub elapsed_time: std::time::Duration,
    pub complete_statistics: CompleteTraversalStatistics,
    pub largest_files: Vec<FileInfo>,
    pub duplicate_groups: Vec<(String, Vec<FileInfo>)>,
}

enum OutputFormat {
    Headers,
    Numbers,
    FilePaths,
    FileSizes,
}

impl OutputFormat {
    fn color(&self) -> Color {
        match self {
            OutputFormat::Headers => Color::BrightBlue,
            OutputFormat::Numbers => Color::Blue,
            OutputFormat::FilePaths => Color::Cyan,
            OutputFormat::FileSizes => Color::Magenta,
        }
    }
}

// const TITLES_COLOR: CustomColor = CustomColor::new(120, 120, 120);

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
            println!(
                "{}: {}",
                file.path
                    .display()
                    .to_string()
                    .color(OutputFormat::FilePaths.color()),
                format_size(file.size as usize).color(OutputFormat::FileSizes.color())
            );
        }

        println!();
        println!(
            "{}",
            "Duplicated files:"
                .bold()
                .underline()
                .color(OutputFormat::Headers.color())
        );

        println!();
        for (hash, group) in &self.duplicate_groups {
            println!("Hash: {}", hash);
            for file in group {
                println!(
                    "{}: {}",
                    file.path
                        .display()
                        .to_string()
                        .color(OutputFormat::FilePaths.color()),
                    format_size(file.size as usize).color(OutputFormat::FileSizes.color())
                );
            }
            println!();
        }
    }
}
