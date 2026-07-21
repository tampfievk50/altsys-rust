use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreateToolCommand::CreateToolCommand;
use crate::dto::ToolResponse::ToolResponse;
use crate::dto::UpdateToolCommand::UpdateToolCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait ToolPort: Send + Sync {
    async fn create_tool(&self, command: CreateToolCommand) -> Result<ToolResponse, DomainError>;
    async fn find_tool_by_id(&self, id: Uuid) -> Result<ToolResponse, DomainError>;
    /// Returns tools scoped to `tenant_id` plus platform-wide (tenant_id = NULL) tools.
    async fn find_tools_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ToolResponse>, DomainError>;
    async fn update_tool(&self, id: Uuid, command: UpdateToolCommand) -> Result<ToolResponse, DomainError>;
    async fn delete_tool(&self, id: Uuid) -> Result<(), DomainError>;
}
