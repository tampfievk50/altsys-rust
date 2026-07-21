use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::Tool::Tool;
use sdlc_domain::port::output::ToolRepositoryPort::ToolRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::tool::mapper::ToolDataMapper::ToolDataMapper;
use crate::tool::repository::ToolSeaOrmRepository::ToolSeaOrmRepository;

pub struct ToolRepositoryImpl {
    sea_orm_repo: ToolSeaOrmRepository,
}

impl ToolRepositoryImpl {
    pub fn new(sea_orm_repo: ToolSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl ToolRepositoryPort for ToolRepositoryImpl {
    async fn save(&self, tool: &Tool) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(ToolDataMapper::to_active_model(tool)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save tool"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, tool: &Tool) -> Result<(), DomainError> {
        self.sea_orm_repo.update(ToolDataMapper::to_active_model(tool)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update tool"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tool>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(ToolDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find tool"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Tool>, DomainError> {
        self.sea_orm_repo.find_by_tenant_including_global(tenant_id).await
            .map(|tools| tools.iter().map(ToolDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list tools"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete tool"); DomainError::InternalError(e.to_string()) })
    }
}
