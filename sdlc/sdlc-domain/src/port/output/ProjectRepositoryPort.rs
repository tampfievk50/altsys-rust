use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::Project::Project;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait ProjectRepositoryPort: Send + Sync {
    async fn save(&self, project: &Project) -> Result<(), DomainError>;
    async fn update(&self, project: &Project) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Project>, DomainError>;
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Project>, DomainError>;
    async fn find_by_slug_and_tenant(&self, slug: &str, tenant_id: Uuid) -> Result<Option<Project>, DomainError>;
    /// Cross-tenant: used by `JiraPollingScheduler` to find every project with a
    /// Jira config to poll, regardless of which tenant owns it.
    async fn find_all_with_jira_tool(&self) -> Result<Vec<Project>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;
}
