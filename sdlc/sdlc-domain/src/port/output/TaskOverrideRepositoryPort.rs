use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::TaskOverride::TaskOverride;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait TaskOverrideRepositoryPort: Send + Sync {
    async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<TaskOverride>, DomainError>;
    async fn find_by_project_and_ticket(&self, project_id: Uuid, ticket_key: &str) -> Result<Option<TaskOverride>, DomainError>;
    async fn save(&self, task_override: &TaskOverride) -> Result<(), DomainError>;
    async fn update(&self, task_override: &TaskOverride) -> Result<(), DomainError>;
}
