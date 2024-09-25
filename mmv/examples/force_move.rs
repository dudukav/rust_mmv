use mmv::args::CLI;
use mmv::mmv::mmv;
use mmv::errors::MassMoveError;
use tempfile::TempDir;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), MassMoveError> {

    let dir = TempDir::new().unwrap();
    let source_file = dir.path().join("some_part_filename.txt");
    let exist_file = dir.path().join("changed_part_filename.txt");

    let mut source = File::create(&source_file).unwrap();
    writeln!(source, "This is a test file.").unwrap();
    let mut exist = File::create(&exist_file).unwrap();
    writeln!(exist, "Test existing file.").unwrap();

    let args = CLI {
        source_pattern: format!("{}/some_*_filename.txt", dir.path().display()),
        destination_pattern: format!("{}/changed_#1_filename.txt", dir.path().display()),
        force: true,
    };

    mmv(args)?;

    Ok(())

}