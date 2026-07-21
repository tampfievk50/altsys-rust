use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::Plugin::Plugin;
use sdlc_domain::port::output::PluginRepositoryPort::PluginRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::plugin::mapper::PluginDataMapper::PluginDataMapper;
use crate::plugin::repository::PluginSeaOrmRepository::PluginSeaOrmRepository;

pub struct PluginRepositoryImpl {
    sea_orm_repo: PluginSeaOrmRepository,
}

impl PluginRepositoryImpl {
    pub fn new(sea_orm_repo: PluginSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl PluginRepositoryPort for PluginRepositoryImpl {
    async fn save(&self, plugin: &Plugin) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(PluginDataMapper::to_active_model(plugin)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to save plugin"); DomainError::InternalError(e.to_string()) })
    }

    async fn update(&self, plugin: &Plugin) -> Result<(), DomainError> {
        self.sea_orm_repo.update(PluginDataMapper::to_active_model(plugin)).await
            .map(|_| ()).map_err(|e| { error!(error = %e, "Failed to update plugin"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Plugin>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(PluginDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find plugin"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Plugin>, DomainError> {
        self.sea_orm_repo.find_by_tenant_including_global(tenant_id).await
            .map(|plugins| plugins.iter().map(PluginDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list plugins"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete plugin"); DomainError::InternalError(e.to_string()) })
    }
}
