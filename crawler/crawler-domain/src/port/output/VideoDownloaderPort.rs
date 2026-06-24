use async_trait::async_trait;

use crate::r#enum::DomainError::DomainError;

#[derive(Debug, Clone)]
pub struct DownloadResult {
    pub file_path: String,
    pub file_size_bytes: u64,
    pub title: Option<String>,
}

#[async_trait]
pub trait VideoDownloaderPort: Send + Sync {
    async fn download(&self, url: &str, download_dir: &str) -> Result<DownloadResult, DomainError>;
}
