use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Download not found: {0}")]
    NotFound(String),

    #[error("Download already in progress: {0}")]
    AlreadyInProgress(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
