pub extern crate clap;
use clap::Parser;

/// Struct to parse CLI arguments.
/// # Arguments
/// - `source_pattern` - the pattern by which the search will take place. It should contain only * and only in filename. Example: 'path/to/some_*_filename.*'
/// - `destination_pattern` - the pattern by which files will be renamed. It sshould contains markers only in filename. Example: 'path2/to/changed_#1_filename.#2'
/// - `force` - flag of CLI app, that overwrites existing files if they exist.
/// # Example
/// ```rust
/// use mmv::args::CLI;
/// use tempfile::TempDir;
/// use std::fs::File;
/// use std::io::Write;
/// 
/// fn main() {
/// let dir = TempDir::new().unwrap();
/// let source_file = dir.path().join("some_part_filename.txt");
/// let destination_file = dir.path().join("changed_part_filename.txt");
///
/// let mut source = File::create(&source_file).unwrap();
/// writeln!(source, "This is a test file.").unwrap();
/// let args = CLI {
///     source_pattern: format!("{}/some_*_filename.txt", dir.path().display()),
///     destination_pattern: format!("{}/changed_#1_filename.txt", dir.path().display()),
///     force: false,
///     };
/// }
/// ```
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
