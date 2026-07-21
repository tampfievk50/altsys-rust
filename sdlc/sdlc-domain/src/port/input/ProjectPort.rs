use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreateProjectCommand::CreateProjectCommand;
use crate::dto::ProjectResponse::ProjectResponse;
use crate::dto::UpdateProjectCommand::UpdateProjectCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait ProjectPort: Send + Sync {
    async fn create_project(&self, command: CreateProjectCommand) -> Result<ProjectResponse, DomainError>;
    async fn find_project_by_id(&self, id: Uuid) -> Result<ProjectResponse, DomainError>;
    async fn find_projects_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ProjectResponse>, DomainError>;
    async fn update_project(&self, id: Uuid, command: UpdateProjectCommand) -> Result<ProjectResponse, DomainError>;
    async fn delete_project(&self, id: Uuid) -> Result<(), DomainError>;
}
