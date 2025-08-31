use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum GitTypeError {
    #[error("Repository path does not exist: {0}")]
    RepositoryNotFound(PathBuf),

    #[error("No supported files found in repository")]
    NoSupportedFiles,

    #[error("Failed to extract code chunks: {0}")]
    ExtractionFailed(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Glob pattern error: {0}")]
    GlobPatternError(#[from] glob::PatternError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Terminal error: {0}")]
    TerminalError(String),

    #[error("Walk directory error: {0}")]
    WalkDirError(#[from] walkdir::Error),
}

pub type Result<T> = std::result::Result<T, GitTypeError>;
