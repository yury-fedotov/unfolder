use colored::Color;

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
            OutputFormat::Numbers => Color::Blue,
            OutputFormat::FilePaths => Color::Cyan,
            OutputFormat::FileSizes => Color::Magenta,
        }
    }
}
