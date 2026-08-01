use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::JobExecution::JobExecution;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait ExecutionRepositoryPort: Send + Sync {
    async fn save(&self, execution: &JobExecution) -> Result<(), DomainError>;
    async fn update(&self, execution: &JobExecution) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<JobExecution>, DomainError>;
    async fn find_by_scheduler_id(&self, scheduler_id: Uuid) -> Result<Vec<JobExecution>, DomainError>;
}
