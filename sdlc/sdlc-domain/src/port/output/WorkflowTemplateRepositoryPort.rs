use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::WorkflowTemplate::WorkflowTemplate;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait WorkflowTemplateRepositoryPort: Send + Sync {
    async fn save(&self, template: &WorkflowTemplate) -> Result<(), DomainError>;
    async fn update(&self, template: &WorkflowTemplate) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkflowTemplate>, DomainError>;
    async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<WorkflowTemplate>, DomainError>;
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowTemplate>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
