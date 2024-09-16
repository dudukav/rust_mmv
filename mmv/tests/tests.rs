#[cfg(test)]
mod tests {
    use mmv::{
        args::CLI,
        errors::MassMoveError,
        fs_utils::{capture_files_by_pattern, rename_file_by_pattern},
        mmv::mmv,
    };
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_capture_file_by_pattern() -> Result<(), std::io::Error> {
        let file_types: Vec<&str> = vec!["bin", "txt", "jpeg"];
        let src_dir = TempDir::new()?;
        for expansion in file_types {
            File::create(src_dir.path().join(format!("tests.{}", expansion)))?;
        }

        let result = capture_files_by_pattern("t*.*");
        assert!(result.is_ok());
        let captured_files = result.unwrap();
        assert_eq!(captured_files, vec!["tests.bin", "tests.txt", "tests.jpeg"]);

        let result = capture_files_by_pattern("abcdef/*.txt");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error, MassMoveError::NotFoundError(_)));

        Ok(())
    }

    #[test]
    fn test_rename_file_by_pattern_simple_case() {
        let source_pattern = "file_*.txt";
        let path = "file_123.txt";
        let destination_pattern = "renamed_file_#1.txt";

        let result = rename_file_by_pattern(source_pattern, path, destination_pattern);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "renamed_file_123.txt");
    }

    #[test]
    fn test_rename_file_by_pattern_multiple_wildcards() {
        let source_pattern = "file_*-v*.txt";
        let path = "file_abc-v123.txt";
        let destination_pattern = "renamed_file_#1_version_#2.txt";

        let result = rename_file_by_pattern(source_pattern, path, destination_pattern);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "renamed_file_abc_version_123.txt");
    }

    #[test]
    fn test_rename_file_by_pattern_no_wildcards_in_source() {
        let source_pattern = "file.txt";
        let path = "file.txt";
        let destination_pattern = "renamed_file.txt";

        let result = rename_file_by_pattern(source_pattern, path, destination_pattern);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "renamed_file.txt");
    }

    #[test]
    fn test_rename_file_by_pattern_no_match() {
        let source_pattern = "file_*.txt";
        let path = "different_file.txt";
        let destination_pattern = "renamed_file_#1.txt";

        let result = rename_file_by_pattern(source_pattern, path, destination_pattern);
        assert!(result.is_err());
        if let Err(MassMoveError::MatchError(message)) = result {
            assert!(message.contains("Pattern {} could not match the path {}."));
        } else {
            panic!("Expected a MatchError.");
        }
    }

    #[test]
    fn test_rename_file_by_pattern_invalid_source_pattern() {
        let source_pattern = "dir_*/file_*.txt";
        let path = "dir_abc/file_123.txt";
        let destination_pattern = "renamed_dir_#1/renamed_file_#2.txt";

        let result = rename_file_by_pattern(source_pattern, path, destination_pattern);
        assert!(result.is_err());
        if let Err(MassMoveError::PathError(message)) = result {
            assert_eq!(
                message,
                "Invalid pattern entered. The pattern should only contain * in the file name."
            );
        } else {
            panic!("Expected a PathError.");
        }
    }

    #[test]
    fn test_rename_file_by_pattern_with_different_extension() {
        let source_pattern = "image_*.jpg";
        let path = "image_456.jpg";
        let destination_pattern = "picture_#1.png"; // Замена расширения

        let result = rename_file_by_pattern(source_pattern, path, destination_pattern);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "picture_456.png");
    }

    #[test]
    fn test_rename_file_by_pattern_edge_case_with_multiple_captures() {
        let source_pattern = "*_file_v*.ext";
        let path = "sample_file_v2.ext";
        let destination_pattern = "prefix_#1_file_version_#2.newext";

        let result = rename_file_by_pattern(source_pattern, path, destination_pattern);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "prefix_sample_file_version_2.newext");
    }

    #[test]
    fn test_rename_file_by_pattern_missing_wildcard_in_path() {
        let source_pattern = "file_*.txt";
        let path = "file.txt";
        let destination_pattern = "renamed_file_#1.txt";

        let result = rename_file_by_pattern(source_pattern, path, destination_pattern);
        assert!(result.is_err());
        if let Err(MassMoveError::MatchError(message)) = result {
            assert!(message.contains("Pattern {} could not match the path {}."));
        } else {
            panic!("Expected a MatchError.");
        }
    }

    #[test]
    fn test_mmv_basic() {
        let tmp_dir = TempDir::new().unwrap();
        let src_file_path = tmp_dir.path().join("file_123.txt");
        let mut src_file = File::create(src_file_path).unwrap();
        writeln!(src_file, "This is a test file.");
    }
}
