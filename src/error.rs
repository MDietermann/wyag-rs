use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum WyagError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("not a git repository (or any parent up to the filesystem root): {0}")]
    NotARepository(PathBuf),

    #[error("repository already exists: {0}")]
    RepositoryExists(PathBuf),
}

pub type Result<T> = std::result::Result<T, WyagError>;
