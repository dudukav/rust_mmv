/// This module is responsible for parsing and managing command-line arguments for the `mmv` (mass mover) application.
///
/// The `args` module defines the structure and logic needed to capture and validate command-line input. It helps configure the source pattern, destination pattern, and various options such as force mode for file moving operations.
///
/// # Structs:
/// - `CLI`: Represents the command-line interface arguments. It holds the following fields:
///     - `source_pattern`: A string that defines the pattern for selecting files (e.g., `path/to/files_*.txt`).
///     - `destination_pattern`: A string defining how to rename or move the selected files (e.g., `new_path/to/files_#1.txt`).
///     - `force`: A boolean flag indicating whether to overwrite files in the destination if they already exist.
/// # Usage:
/// - This module uses a library such as `clap` or a custom argument parser to define and retrieve arguments from the command line.
/// - Arguments include patterns for matching files, destination paths for renaming or moving files, and options like `--force` to control the overwrite behavior.
/// - The parsed arguments are passed to the main logic of the `mmv` program, where they are used to execute file operations.
///
/// # Common Options:
/// - `--source-pattern`: Defines the files to capture based on the provided pattern.
/// - `--destination-pattern`: Specifies how the captured files should be renamed and where they should be moved.
/// - `--force`: Optional flag to allow overwriting files that already exist in the destination path.
///
/// This module ensures that command-line input is correctly handled and validated before being used in the core file moving logic.
pub mod args;
/// This module defines custom error types for handling various failure scenarios within the application.
///
/// The `errors` module provides custom error handling for the `mmv` (mass mover) program. It defines different error types that cover all possible issues encountered during file pattern matching, renaming, and moving operations.
///
/// # Error Types:
/// - `MassMoveError`: The primary error enum that encompasses different error types, including:
///     - `IoError`: Represents input/output errors that occur during file system operations (e.g., moving, renaming, copying files).
///     - `NotFoundError`: Raised when no files match the provided source pattern.
///     - `MatchError`: Raised when the source pattern does not match any part of the file or if renaming fails.
///     - `FileExistsError`: Raised when a file already exists in the destination and the `force` option is not enabled.
/// # Usage:
/// - The `MassMoveError` enum provides a centralized way of handling errors in the application.
/// - When an error occurs, the appropriate variant is returned and can be matched on to handle specific cases or propagate the error up the stack.
/// 
/// This approach enables cleaner error handling and more informative error messages, improving the debugging experience for developers and users of the program.
pub mod errors;
/// Moves and renames multiple files based on the provided source and destination patterns.
/// 
/// # Arguments
/// - `args` - A `CLI` struct containing the following:
///     - `source_pattern` - A pattern to capture files from the source directory, e.g., `"path/to/files_*.txt"`.
///     - `destination_pattern` - A pattern to rename and move the captured files to the destination directory, e.g., `"new_path/to/renamed_#1.txt"`.
///     - `force` - A boolean indicating whether to overwrite files if they already exist in the destination.
/// 
/// The function:
/// 1. Finds all files matching the `source_pattern`.
/// 2. Renames each captured file according to the `destination_pattern`.
/// 3. Moves each file to the destination directory.
/// 4. Prints the source and destination paths for each file moved.
/// # Errors
/// - Returns `MassMoveError::NotFoundError` if no files match the `source_pattern`.
/// - Returns `MassMoveError::MatchError` if a file does not match the renaming pattern.
/// - Returns `MassMoveError::FileExistsError` if a destination file exists and the `force` flag is `false`.
/// - Returns `MassMoveError::IoError` for any I/O issues during file operations (moving, renaming, etc.).
/// 
/// # Behavior
/// - The function will stop and return the first error encountered (e.g., if one of the files cannot be renamed or moved).
/// - If `force` is set to `true`, existing files in the destination directory will be overwritten.
pub mod fs_utils;
/// This module provides the core functionality for the `mmv` (mass mover) application.
/// It handles the process of finding, renaming, and moving files based on user-defined patterns.
///
/// The `mmv` module orchestrates the key operations for bulk file manipulation:
/// capturing files that match a source pattern, renaming those files according to a specified
/// destination pattern, and finally moving or renaming the files to their new locations.
///
/// It integrates the functionality from other modules, such as `fs_utils` for file operations
/// and `args` for handling command-line arguments.
///
/// # Main Function:
///
/// - `mmv`: This function is the main entry point of the `mmv` application logic. It uses patterns
///   provided by the user to find matching files, rename them, and move them to a new destination.
///   
///   - **Arguments**:
///     - `args` (`CLI`): A structure containing the parsed command-line arguments, including:
///       - `source_pattern`: The pattern to find files that need to be moved.
///       - `destination_pattern`: The pattern for renaming and moving the matched files.
///       - `force`: A boolean flag that determines whether existing files should be overwritten.
///   
///   - **Returns**: `Result<(), MassMoveError>`: The function returns an empty result if all operations
///     are successful, or an error if any part of the process fails (e.g., no files match the pattern, the file can't be moved).
///
/// # Usage:
/// The `mmv` module is the core of the `mmv` program, responsible for coordinating file discovery, renaming, and relocation.
/// It takes user input in the form of file patterns and uses helper modules to execute the file operations.
/// This module is the backbone of the application, integrating pattern matching, error handling, and file system operations.
/// # Workflow:
///
/// 1. **Capture Files**: The `mmv` function first captures all files that match the source pattern using
///    the `capture_files_by_pattern` function from the `fs_utils` module.
/// 
/// 2. **Rename Files**: For each matched file, it applies the renaming rules specified in the destination pattern using
///    the `rename_file_by_pattern` function, replacing placeholders like `#1` with corresponding parts of the original filename.
/// 
/// 3. **Move Files**: After renaming, the `move_file` function is called to move the files to their new locations.
///    If the destination file exists, it checks the `force` flag to decide whether to overwrite the file.
///
/// # Error Handling:
/// The `mmv` function ensures proper error handling at every step, including:
/// - Handling patterns that match no files.
/// - Errors during file renaming or moving (such as permission issues or file system errors).
/// - Graceful handling of file conflicts (when `force` is not enabled and the destination file already exists).
pub mod mmv;
