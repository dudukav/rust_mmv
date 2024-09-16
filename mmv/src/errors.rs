/// Enumuration of errors fo Mass Mover Project.
#[derive(Debug)]
pub enum MassMoveError {
    /// Error, if there are no files matched the source pattern in directory.
    NotFoundError(String),
    /// Glob Pattern Error implemented for Mass Mover.
    PatternError(glob::PatternError),
    /// Error, if parrent path contains markers.
    PathError(String),
    /// std IO Error implemented for Mass Mover.
    StdIOError(std::io::Error),
    /// Error, if file is already exists.
    FileExistsError(String),
    /// Regex Error implemented for Mass Mover.
    RegexError(regex::Error),
    /// Error, if there are no matches between a source pattern and a path.
    MatchError(String),
}

/// Implementation of Glob Pattern Error for Mass Mover.
impl From<glob::PatternError> for MassMoveError {
    fn from(error: glob::PatternError) -> MassMoveError {
        MassMoveError::PatternError(error)
    }
}

/// Implementation of std IO Error for Mass Mover.
impl From<std::io::Error> for MassMoveError {
    fn from(err: std::io::Error) -> MassMoveError {
        MassMoveError::StdIOError(err)
    }
}

/// Implementation of Regex Error for Mass Mover.
impl From<regex::Error> for MassMoveError {
    fn from(err: regex::Error) -> MassMoveError {
        MassMoveError::RegexError(err)
    }
}
