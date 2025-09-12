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

    #[error("Repository clone error: {0}")]
    RepositoryCloneError(#[from] git2::Error),

    #[error("Invalid repository format: {0}")]
    InvalidRepositoryFormat(String),

    #[error("Tree-sitter language error: {0}")]
    TreeSitterLanguageError(#[from] tree_sitter::LanguageError),

    #[error("Application panic: {0}")]
    PanicError(String),
}

impl GitTypeError {
    /// Create a custom database error from a string message
    pub fn database_error(msg: String) -> Self {
        Self::DatabaseError(rusqlite::Error::ToSqlConversionFailure(Box::new(
            std::io::Error::other(msg),
        )))
    }
}

pub type Result<T> = std::result::Result<T, GitTypeError>;
