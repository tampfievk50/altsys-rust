use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::Plugin::Plugin;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait PluginRepositoryPort: Send + Sync {
    async fn save(&self, plugin: &Plugin) -> Result<(), DomainError>;
    async fn update(&self, plugin: &Plugin) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Plugin>, DomainError>;
    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Plugin>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
