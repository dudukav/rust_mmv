use crate::args::CLI;
use crate::errors::MassMoveError;
use crate::fs_utils::{capture_files_by_pattern, move_file, rename_file_by_pattern};

pub fn mmv(args: CLI) -> Result<(), MassMoveError> {
    let captured_files = capture_files_by_pattern(&args.source_pattern)?;

    for source_file in captured_files {
        let renamed_file = rename_file_by_pattern(
            &args.source_pattern,
            &source_file,
            &args.destination_pattern,
        )?;
        move_file(&source_file, &renamed_file, &args.force)?;
        println!("{} -> {}", source_file, renamed_file);
    }

    Ok(())
}
