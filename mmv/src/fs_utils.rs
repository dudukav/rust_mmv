extern crate glob;
use crate::errors::MassMoveError;
use glob::glob;
use regex::Regex;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

/// This function searches files in global directory by pattern.
/// # Argumets:
/// - `pattern` - it is a filename pattern used to capture multiple files in a directory in format “path/to/dir/files_*.txt”, where you can specify multiple asterisks. Pattern should contain "*" only in filename.
/// # Return value
/// Returns `Result<HashSet<String>, MassMoveError>`, where:
/// - If successful - returns many file paths corresponding to the template.
/// - In case of an error, a `MassMoveError` type error occurs if no file was found or another error occurred.
/// # Example:
/// ```rust
/// use mmv::errors::MassMoveError;
/// use mmv::fs_utils::capture_files_by_pattern;
/// use std::collections::HashSet;
/// use std::fs::File;
/// use tempfile::tempdir;
/// fn main() -> Result<(), MassMoveError> {
///     let temp_dir = tempdir()?;
///     let temp_path = temp_dir.path();
///
///     File::create(temp_path.join("test.txt"))?;
///     File::create(temp_path.join("test.bin"))?;
///
///     let pattern = format!("{}/test.*", temp_path.display());
///
///     let matched_files = capture_files_by_pattern(&pattern)?;
/// 
///     let expected_files: HashSet<String> = [
///         format!("{}/test.txt", temp_path.display()),
///         format!("{}/test.bin", temp_path.display()),
///     ]
///     .iter()
///     .cloned()
///     .collect();
///
///     assert_eq!(matched_files, expected_files);
///
///     Ok(())
/// }
/// ```
/// # Possible errors
/// - `MassMoveError::NotFoundError` - error, if there are no files in directory matched the pattern.
/// - `GlobError` - error, when a particular path cannot be read to determine if its contents match the glob pattern.
pub fn capture_files_by_pattern(pattern: &str) -> Result<HashSet<String>, MassMoveError> {
    let mut files_by_pattern: HashSet<String> = HashSet::new();

    for entry in glob(pattern)? {
        if let Ok(path) = entry {
            files_by_pattern.insert(path.display().to_string().replace("\\", "/"));
        }
    }

    if files_by_pattern.is_empty() {
        return Err(MassMoveError::NotFoundError(
            "No matches for this pattern.".to_string(),
        ));
    }

    Ok(files_by_pattern)
}

fn path_check(pattern: &str, marker: &str) -> Result<(), MassMoveError> {
    let parent_path = Path::new(pattern)
        .parent()
        .unwrap()
        .to_string_lossy()
        .to_string();

    if parent_path.contains(marker) {
        return Err(MassMoveError::PathError(
            "Invalid pattern entered. The pattern should only contain * in the file name."
                .to_string(),
        ));
    }

    Ok(())
}

/// This function creates a new name based on the pattern by which the file was found, the path to the file itself, and the pattern by which the path should be changed.
/// # Return value
/// Returns `Result<(), MassMoveerror>`, where:
/// - If succesful - returns a new filenamr.
/// - In case of an error, a `MassMoveError` type error occurs if invalid patterns were given or path does not match the source pattern.
/// # Argumets
/// - `source_pattern` - a filename pattern used to capture multiple files in a directory in format “path/to/dir/files_*.txt”, where you can specify multiple asterisks. Pattern should contain '*' only in filename.
/// - `path` - a path of file, that will be changed by pattern.
/// - `destination_pattern` - a filename pattern used to change filename and file path in directory in format "path/to/dir/files_new#1.txt". Pattern should contain '#' only in filename.
/// # Example:
/// ```rust
/// use mmv::fs_utils::rename_file_by_pattern;
/// use mmv::errors::MassMoveError;
/// 
/// fn main() -> Result<(), MassMoveError> {
///     let source_pattern = "some_dir/dir/t*.*";
///     let path = "some_dir/dir/test.bin";
///     let destination_pattern = "some_dir_2/dir_2/foo_#1.bin";
///     let result = rename_file_by_pattern(&source_pattern, &path, &destination_pattern)?;
/// 
///     assert_eq!(result, "some_dir_2/dir_2/foo_est.bin");
///     
///     Ok(())
/// }
/// ```
/// # Possible errors
/// - `MassMove::PathError` 
/// - `massMove::MatchError`
pub fn rename_file_by_pattern(
    source_pattern: &str,
    path: &str,
    destination_pattern: &str,
) -> Result<String, MassMoveError> {
    path_check(source_pattern, "*")?;
    path_check(destination_pattern, "#")?;

    let source_str = source_pattern.replace(".", "\\.").replace("*", "(.*)");
    let regex = Regex::new(&source_str)?;
    if let Some(captures) = regex.captures(path) {
        let mut new_path = destination_pattern.to_owned();
        for i in 1..captures.len() {
            let marker = format!("#{}", i);
            let replacement = captures.get(i).map_or("", |m| m.as_str());
            new_path = new_path.replacen(&marker, &replacement, 1);
        }
        Ok(new_path)
    } else {
        Err(MassMoveError::MatchError(format!(
            "Pattern {} could not match the path {}.",
            source_pattern, path
        )))
    }
}


/// This function moves the content from one file to other.
/// # Arguments
/// - `source_file` - the file whose content wil be moved.
/// - `destination_file` - the file where contenr will be moved.
/// - `force` - flag for overwriting existing files if they exist.
/// # Return value
/// Returns Result<(), MassMoveError> where:
/// - Returns nothing if function complete succesfully.
/// - Returns error if not
/// # Example
/// ```rust
/// use std::io::Write;
/// use mmv::errors::MassMoveError;
/// use mmv::fs_utils::move_file;
/// use std::path::PathBuf;
/// use std::fs::{File, write};
/// use tempfile::TempDir;
///
/// fn main() -> Result<(), MassMoveError> {
///     let dir = TempDir::new().unwrap();
///     let source_file = dir.path().join("some_part_filename.txt");
///     let destination_file = dir.path().join("changed_part_filename.txt");

///     let mut source = File::create(&source_file).unwrap();
///     writeln!(source, "This is a test file.").unwrap();
///
///     let result = move_file(&source_file, &destination_file, &false);
///     result
/// }
/// ```
/// # Errors
/// - `MassMoveError::FileExistsError` - If the destination file already exists and force is not set.
/// - `MassMoveError::IoError` - For any other I/O error during file operations (e.g., renaming, copying, or deleting).
pub fn move_file(
    source_file: &PathBuf,
    destination_file: &PathBuf,
    force: &bool,
) -> Result<(), MassMoveError> {
    if !force && destination_file.exists() {
        return Err(MassMoveError::FileExistsError(
            "The file already exists. Try --force mode to overwrite the file".to_string(),
        ));
    }

    match fs::rename(&source_file, &destination_file) {
        Ok(_) => Ok(()),
        Err(_) => {
            fs::copy(&source_file, &destination_file)?;
            fs::remove_file(&source_file)?;
            Ok(())
        }
    }
}
