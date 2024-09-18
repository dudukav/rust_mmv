use std::path::PathBuf;

use crate::args::CLI;
use crate::errors::MassMoveError;
use crate::fs_utils::{capture_files_by_pattern, move_file, rename_file_by_pattern};

/// Moves and renames multiple files based on the provided source and destination patterns.
/// # Arguments
/// - `args` - A `CLI` struct containing the following:
///     - `source_pattern` - A pattern to capture files from the source directory, e.g., `"path/to/files_*.txt"`.
///     - `destination_pattern` - A pattern to rename and move the captured files to the destination directory, e.g., `"new_path/to/renamed_#1.txt"`.
///     - `force` - A boolean indicating whether to overwrite files if they already exist in the destination.
/// The function:
/// 1. Finds all files matching the `source_pattern`.
/// 2. Renames each captured file according to the `destination_pattern`.
/// 3. Moves each file to the destination directory.
/// 4. Prints the source and destination paths for each file moved.
/// # Example
/// ```rust
/// use tempfile::TempDir;
/// use std::io::Write;
/// use std::fs::File;
/// use mmv::{errors::MassMoveError, args::CLI, mmv::mmv};
///
/// fn main() -> Result<(), MassMoveError> {
///     let dir = TempDir::new().unwrap();
///     let source_file = dir.path().join("some_part_filename.txt");
///     let destination_file = dir.path().join("changed_part_filename.txt");
///
///     let mut source = File::create(&source_file).unwrap();
///     writeln!(source, "This is a test file.").unwrap();

///     let args = CLI {
///         source_pattern: format!("{}/some_*_filename.txt", dir.path().display()),
///         destination_pattern: format!("{}/changed_#1_filename.txt", dir.path().display()),
///         force: false,
///     };
///
///     let result = mmv(args);
///     assert!(result.is_ok());
///     assert!(!source_file.exists());
///     result
/// }
/// ```
/// # Errors
/// - Returns `MassMoveError::NotFoundError` if no files match the `source_pattern`.
/// - Returns `MassMoveError::MatchError` if a file does not match the renaming pattern.
/// - Returns `MassMoveError::FileExistsError` if a destination file exists and the `force` flag is `false`.
/// - Returns `MassMoveError::IoError` for any I/O issues during file operations (moving, renaming, etc.).
/// # Behavior
/// - The function will stop and return the first error encountered (e.g., if one of the files cannot be renamed or moved).
/// - If `force` is set to `true`, existing files in the destination directory will be overwritten.

pub fn mmv(args: CLI) -> Result<(), MassMoveError> {
    let captured_files = capture_files_by_pattern(&args.source_pattern)?;

    for source_file in captured_files {
        let renamed_file = rename_file_by_pattern(
            &args.source_pattern,
            &source_file,
            &args.destination_pattern,
        )?;
        let source_path = PathBuf::from(&source_file);
        let destination_path = PathBuf::from(&renamed_file);
        move_file(&source_path, &destination_path, &args.force)?;
        println!("{} -> {}", source_file, renamed_file);
    }

    Ok(())
}
