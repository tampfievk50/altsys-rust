use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use crawler_domain::dto::VideoDownload::VideoDownload;
use crawler_domain::port::output::DownloadRepositoryPort::DownloadRepositoryPort;
use crawler_domain::r#enum::DomainError::DomainError;

use crate::download::mapper::DownloadDataMapper::DownloadDataMapper;
use crate::download::repository::DownloadSeaOrmRepository::DownloadSeaOrmRepository;

pub struct DownloadRepositoryImpl {
    sea_orm_repo: DownloadSeaOrmRepository,
}

impl DownloadRepositoryImpl {
    pub fn new(sea_orm_repo: DownloadSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl DownloadRepositoryPort for DownloadRepositoryImpl {
    async fn save(&self, download: &VideoDownload) -> Result<(), DomainError> {
        let active_model = DownloadDataMapper::to_active_model(download);
        self.sea_orm_repo.insert(active_model).await.map_err(|e| {
            error!(error = %e, "Failed to save download via SeaORM");
            DomainError::DownloadFailed(format!("Failed to save download: {}", e))
        })?;
        Ok(())
    }

    async fn update(&self, download: &VideoDownload) -> Result<(), DomainError> {
        let active_model = DownloadDataMapper::to_active_model(download);
        self.sea_orm_repo.update(active_model).await.map_err(|e| {
            error!(error = %e, "Failed to update download via SeaORM");
            DomainError::DownloadFailed(format!("Failed to update download: {}", e))
        })?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<VideoDownload>, DomainError> {
        let model = self.sea_orm_repo.find_by_id(id).await.map_err(|e| {
            error!(error = %e, "Failed to find download via SeaORM");
            DomainError::DownloadFailed(format!("Failed to query download: {}", e))
        })?;

        Ok(model.as_ref().map(DownloadDataMapper::to_domain))
    }

    async fn find_all(&self) -> Result<Vec<VideoDownload>, DomainError> {
        let models = self.sea_orm_repo.find_all().await.map_err(|e| {
            error!(error = %e, "Failed to list downloads via SeaORM");
            DomainError::DownloadFailed(format!("Failed to list downloads: {}", e))
        })?;

        Ok(models.iter().map(DownloadDataMapper::to_domain).collect())
    }
}
