Returns new name for file created by the pattern. -->
# Argumets
- `source_pattern` - it is a filename pattern used to capture multiple files in a directory in format “path/to/dir/files_*.txt”, where you can specify multiple asterisks.
- `path` - it is a path of file, that will be changed by pattern.
- `destination_pattern` - it is a filename pattern used to change filename and file path in directory in format "path/to/dir/files_new#1.txt".
# Example:
```rust
use regex::Regex;
use std::path::Path;
use crate::errors::MassMoveError;

let source_pattern =  "dir/mini_dir/test.*";
let path = "dir/mini_dir/test.txt";
let destination_pattern = "dir/mini_dir2/funny_test.#1";
let new_path = rename_file_by_pattern(&source_pattern, &path, &destination_pattern);
assert_eq!(new_path, "dir/mini_dir2/funny_test.txt");
```