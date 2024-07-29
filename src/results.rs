use colored::*;
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

        println!(
            "{}",
            format!("Elapsed time: {:?}", self.elapsed_time).blue()
        );
        println!(
            "{}",
            format!("Number of files analyzed: {}", self.file_count).green()
        );
        println!(
            "{}",
            format!("Number of directories traversed: {}", self.dir_count).green()
        );
        println!(
            "{}",
            format!("Deepest level of folder nesting: {}", self.max_depth).green()
        );

        println!();
        println!("{}", "Largest files:".bold().underline().yellow());

        for (file, size) in &self.largest_files {
            println!(
                "{}: {} bytes",
                file.display().to_string().cyan(),
                size.to_string().magenta()
            );
        }
    }
}
