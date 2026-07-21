use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::ProjectContext::ProjectContext;
use crate::r#enum::DomainError::DomainError;

/// Driven adapter for the Platform service (Phase 1): resolves a project's
/// repository/branch/build/test/Jira configuration.
#[async_trait]
pub trait PlatformClientPort: Send + Sync {
    async fn get_project(&self, project_id: Uuid) -> Result<ProjectContext, DomainError>;
}
