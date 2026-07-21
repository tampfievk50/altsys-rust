use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::WorkflowExecution::WorkflowExecution;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait WorkflowExecutionRepositoryPort: Send + Sync {
    async fn save(&self, execution: &WorkflowExecution) -> Result<(), DomainError>;
    async fn update(&self, execution: &WorkflowExecution) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkflowExecution>, DomainError>;
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WorkflowExecution>, DomainError>;
}
