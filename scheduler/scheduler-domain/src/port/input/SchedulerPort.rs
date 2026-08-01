use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreateSchedulerCommand::CreateSchedulerCommand;
use crate::dto::UpdateSchedulerCommand::UpdateSchedulerCommand;
use crate::dto::SchedulerResponse::SchedulerResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait SchedulerPort: Send + Sync {
    async fn create_scheduler(&self, command: CreateSchedulerCommand) -> Result<SchedulerResponse, DomainError>;
    async fn find_scheduler_by_id(&self, id: Uuid) -> Result<SchedulerResponse, DomainError>;
    async fn find_all_schedulers(&self) -> Result<Vec<SchedulerResponse>, DomainError>;
    async fn update_scheduler(&self, id: Uuid, command: UpdateSchedulerCommand) -> Result<SchedulerResponse, DomainError>;
    async fn delete_scheduler(&self, id: Uuid) -> Result<(), DomainError>;
    async fn pause_scheduler(&self, id: Uuid) -> Result<SchedulerResponse, DomainError>;
    async fn resume_scheduler(&self, id: Uuid) -> Result<SchedulerResponse, DomainError>;
}
