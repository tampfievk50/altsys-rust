use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreatePluginCommand::CreatePluginCommand;
use crate::dto::PluginResponse::PluginResponse;
use crate::dto::UpdatePluginCommand::UpdatePluginCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait PluginPort: Send + Sync {
    async fn create_plugin(&self, command: CreatePluginCommand) -> Result<PluginResponse, DomainError>;
    async fn find_plugin_by_id(&self, id: Uuid) -> Result<PluginResponse, DomainError>;
    /// Returns plugins scoped to `tenant_id` plus platform-wide (tenant_id = NULL) plugins.
    async fn find_plugins_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<PluginResponse>, DomainError>;
    async fn update_plugin(&self, id: Uuid, command: UpdatePluginCommand) -> Result<PluginResponse, DomainError>;
    async fn delete_plugin(&self, id: Uuid) -> Result<(), DomainError>;
}
