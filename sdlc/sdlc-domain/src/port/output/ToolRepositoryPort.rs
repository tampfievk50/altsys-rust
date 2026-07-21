use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::Tool::Tool;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait ToolRepositoryPort: Send + Sync {
    async fn save(&self, tool: &Tool) -> Result<(), DomainError>;
    async fn update(&self, tool: &Tool) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tool>, DomainError>;
    /// Returns rows where `tenant_id = tenant_id` OR `tenant_id IS NULL` (global tools).
    async fn find_by_tenant_including_global(&self, tenant_id: Uuid) -> Result<Vec<Tool>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
