
extern crate glob;
use crate::errors::MassMoveError;
use glob::glob;
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[doc = include_str!("../docs/capture_files_by_pattern_doc.md")]
pub fn capture_files_by_pattern(pattern: &str) -> Result<Vec<String>, MassMoveError> {
    let mut files_by_pattern: Vec<String> = Vec::new();

    for entry in glob(pattern)? {
        if let Ok(path) = entry {
            files_by_pattern.push(path.display().to_string().replace("\\", "/"));
        }
    }

    if files_by_pattern.is_empty() {
        return Err(MassMoveError::NotFoundError(
            "No matches for this pattern.".to_string(),
        ));
    }

    Ok(files_by_pattern)
}

#[doc = include_str!("../docs/rename_file_by_pattern_doc.md")]
pub fn rename_file_by_pattern(
source_pattern: &str,
path: &str,
destination_pattern: &str,
) -> Result<String, MassMoveError> {
    let parent_path = Path::new(source_pattern)
        .parent()
        .unwrap()
        .to_string_lossy()
        .to_string();

    if parent_path.contains('*') {
        return Err(MassMoveError::PathError(
            "Invalid pattern entered. The pattern should only contain * in the file name."
                .to_string(),
        ));
    }

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

pub fn move_file(
    source_file: &str,
    destination_file: &str,
    force: &bool,
) -> Result<(), MassMoveError> {
    let source_path = PathBuf::from(source_file);
    let destintaion_path = PathBuf::from(destination_file);

    if !force && destintaion_path.exists() {
        return Err(MassMoveError::FileExistsError(
            "The file already exists. Try --force mode to overwrite the file".to_string(),
        ));
    }

    match fs::rename(&source_path, &destintaion_path) {
        Ok(_) => Ok(()),
        Err(_) => {
            fs::copy(&source_path, &destintaion_path)?;
            fs::remove_file(&source_path)?;
            Ok(())
        }
    }
}
