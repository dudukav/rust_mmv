This method returns list of the files matching the pattern.
# Argumets:
- `pattern` - it is a filename pattern used to capture multiple files in a directory in format “path/to/dir/files_*.txt”, where you can specify multiple asterisks.
# Example:
```rust
 use glob::glob;
 use crate::errors::MassMoveError;

 let pattern = "dir/mini_dir/test.*";
 let matched_files = capture_files_by_pattern(pattern);
 assert_eq!(matched_files, {"dir/mini_dir/test.txt", "dir/mini_dir/test.bin"});
```