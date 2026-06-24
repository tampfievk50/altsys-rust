use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::VideoDownload::VideoDownload;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait DownloadRepositoryPort: Send + Sync {
    async fn save(&self, download: &VideoDownload) -> Result<(), DomainError>;
    async fn update(&self, download: &VideoDownload) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<VideoDownload>, DomainError>;
    async fn find_all(&self) -> Result<Vec<VideoDownload>, DomainError>;
}
