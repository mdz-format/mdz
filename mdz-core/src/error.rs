use thiserror::Error;

/// MDZ error types
#[derive(Error, Debug)]
pub enum MdzError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid MDZ format: {0}")]
    InvalidFormat(String),

    #[error("Missing required file: {0}")]
    MissingFile(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Parsing error: {0}")]
    Parse(String),

    #[error("Rendering error: {0}")]
    Render(String),
}

/// Result type for MDZ operations
pub type Result<T> = std::result::Result<T, MdzError>;