use mmv::args::CLI;
use mmv::mmv::mmv;

fn main() {

    let args = CLI {
        source_pattern: "non_existent_files/*.txt".to_string(),
        destination_pattern: "new_location/new_file_#1.txt".to_string(),
        force: false,
    };

    let result = mmv(args);
    assert!(result.is_err());
}
