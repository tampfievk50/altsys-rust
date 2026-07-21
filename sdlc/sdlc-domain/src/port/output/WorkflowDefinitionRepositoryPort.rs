use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::WorkflowDefinition::WorkflowDefinition;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait WorkflowDefinitionRepositoryPort: Send + Sync {
    async fn save(&self, definition: &WorkflowDefinition) -> Result<(), DomainError>;
    async fn update(&self, definition: &WorkflowDefinition) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkflowDefinition>, DomainError>;
    async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<WorkflowDefinition>, DomainError>;
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowDefinition>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
