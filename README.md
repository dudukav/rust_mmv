# Mass Mover CLI app

Mass Mover is a Rust-based utility designed to efficiently find, rename, and move files based on user-defined patterns. It simplifies batch file operations by using customizable patterns for both source file matching and renaming/moving them to target locations.

## Features
- **Pattern-Based File Matching**: Use wildcard patterns to locate files in specific directories.
- **Flexible Renaming**: Dynamically rename files using custom patterns with placeholders.
- **Batch Moving**: Move files from one location to another in bulk, based on matching patterns.

## How It Works

1. **File Matching**: Specify a source pattern (e.g., path/to/files_*_name.*), where * acts as a wildcard to match parts of file names.
2. **Renaming and Moving**: Define a target pattern (e.g., new_path/to/#1_name.#2) using placeholders (#1, #2, etc.) to map matched segments from the source files to the destination.
3. **Bulk Processing**: Mass Mover will find all files matching the source pattern, rename them according to the target pattern, and move them to the specified location.

## Example

```bash
$ cargo run -- --source-pattern "path/to/some_*_file.*" --destination-pattern "new_path/to/renamed_#1_file.#2"
```

## Installation

1. Clone git repository:
```bash
$ git clone https://github.com/dudukav/rust_mmv.git
```
2. Build the project using Cargo:
```bash
cargo build --release
```

## Documenation 
To generate the documentation, run:
```bash
cargo doc --open
```


