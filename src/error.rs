#[derive(Debug, thiserror::Error)]
pub enum WyagError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, WyagError>;
