use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::ExecutionResponse::ExecutionResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait SchedulerRunnerPort: Send + Sync {
    /// Fires a scheduler immediately, regardless of its cron schedule ("run scheduler by id").
    async fn run_scheduler(&self, id: Uuid) -> Result<ExecutionResponse, DomainError>;

    /// Polled by the background engine: fires every active scheduler whose next_run_at is due.
    async fn tick(&self) -> Result<(), DomainError>;
}
