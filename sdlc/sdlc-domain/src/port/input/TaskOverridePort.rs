use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::TaskOverrideResponse::TaskOverrideResponse;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait TaskOverridePort: Send + Sync {
    async fn find_overrides_by_project(&self, project_id: Uuid) -> Result<Vec<TaskOverrideResponse>, DomainError>;
    /// Creates or replaces the override for `(project_id, ticket_key)`.
    async fn set_summary_override(&self, project_id: Uuid, ticket_key: String, summary: String) -> Result<TaskOverrideResponse, DomainError>;
}
