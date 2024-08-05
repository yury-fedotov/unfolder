use crate::file_sizes::format_size;
use crate::file_utils::FileInfo;
use colored::Color;
use colored::*;

pub enum OutputFormat {
    Headers,
    Numbers,
    FilePaths,
    FileSizes,
}

impl OutputFormat {
    pub fn color(&self) -> Color {
        match self {
            OutputFormat::Headers => Color::BrightBlue,
            OutputFormat::Numbers => Color::Green,
            OutputFormat::FilePaths => Color::Cyan,
            OutputFormat::FileSizes => Color::Magenta,
        }
    }
}

pub fn print_file_path_with_size(file: &FileInfo) {
    println!(
        "- {}: {}",
        file.path
            .display()
            .to_string()
            .color(OutputFormat::FilePaths.color()),
        format_size(file.size as usize).color(OutputFormat::FileSizes.color())
    );
}
