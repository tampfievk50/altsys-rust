use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::dto::Scheduler::Scheduler;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait SchedulerRepositoryPort: Send + Sync {
    async fn save(&self, scheduler: &Scheduler) -> Result<(), DomainError>;
    async fn update(&self, scheduler: &Scheduler) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Scheduler>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Scheduler>, DomainError>;
    async fn find_due(&self, now: DateTime<Utc>) -> Result<Vec<Scheduler>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
