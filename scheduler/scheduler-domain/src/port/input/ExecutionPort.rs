use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::ExecutionResponse::ExecutionResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait ExecutionPort: Send + Sync {
    async fn find_executions_by_scheduler(&self, scheduler_id: Uuid) -> Result<Vec<ExecutionResponse>, DomainError>;
    async fn find_execution_by_id(&self, id: Uuid) -> Result<ExecutionResponse, DomainError>;
}
