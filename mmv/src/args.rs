pub extern crate clap;
use clap::Parser;

/// Struct to parse CLI arguments.
#[derive(Parser, Debug)]
#[command(
    author = "Victoria Kashurkina",
    name = "mmv",
    version = "1.0",
    about = "This is a CLI tool to move and rename
 acll files matched by pattern."
)]
pub struct CLI {
    /// Pattern to search for files in the directory.
    #[arg(long)]
    pub source_pattern: String,

    /// Pattern to rename files.
    #[arg(long)]
    pub destination_pattern: String,

    /// Overwrite existing files
    #[arg(short, long)]
    pub force: bool,
}
