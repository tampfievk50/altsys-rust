use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::DownloadVideoCommand::DownloadVideoCommand;
use crate::dto::DownloadVideoResponse::DownloadVideoResponse;
use crate::dto::VideoDownloadResponse::VideoDownloadResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait DownloadPort: Send + Sync {
    async fn create_download(&self, command: DownloadVideoCommand) -> Result<DownloadVideoResponse, DomainError>;
    async fn find_download_by_id(&self, id: Uuid) -> Result<VideoDownloadResponse, DomainError>;
    async fn find_all_downloads(&self) -> Result<Vec<VideoDownloadResponse>, DomainError>;
}
