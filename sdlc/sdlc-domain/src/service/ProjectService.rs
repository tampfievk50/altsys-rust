use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreateProjectCommand::CreateProjectCommand;
use crate::dto::Project::{NewProject, Project};
use crate::dto::ProjectResponse::ProjectResponse;
use crate::dto::UpdateProjectCommand::UpdateProjectCommand;
use crate::port::input::ProjectPort::ProjectPort;
use crate::port::output::ProjectRepositoryPort::ProjectRepositoryPort;
use crate::port::output::ToolRepositoryPort::ToolRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct ProjectService {
    project_repository: Arc<dyn ProjectRepositoryPort>,
    tool_repository: Arc<dyn ToolRepositoryPort>,
}

impl ProjectService {
    pub fn new(project_repository: Arc<dyn ProjectRepositoryPort>, tool_repository: Arc<dyn ToolRepositoryPort>) -> Self {
        Self { project_repository, tool_repository }
    }

    fn to_response(project: &Project) -> ProjectResponse {
        ProjectResponse {
            id: project.id,
            tenant_id: project.tenant_id,
            name: project.name.clone(),
            slug: project.slug.clone(),
            github_tool_id: project.github_tool_id,
            default_branch: project.default_branch.clone(),
            jira_tool_id: project.jira_tool_id,
            jira_last_synced_at: project.jira_last_synced_at,
            build_command: project.build_command.clone(),
            test_command: project.test_command.clone(),
            coding_standards: project.coding_standards.clone(),
            workflow_config: project.workflow_config.clone(),
            is_active: project.is_active,
            created_at: project.created_at,
            updated_at: project.updated_at,
            created_by: project.created_by,
            updated_by: project.updated_by,
        }
    }

    /// Confirms `tool_id` refers to a Tool of the expected type visible to
    /// `tenant_id` (tenant-owned or global), so a Project can't be linked to a
    /// nonexistent or mistyped config — that mismatch would otherwise only
    /// surface much later, when an SDLC run tries to resolve it.
    async fn validate_tool_reference(&self, tool_id: Uuid, expected_type: &str, tenant_id: Uuid) -> Result<(), DomainError> {
        let tool = self.tool_repository.find_by_id(tool_id).await?
            .ok_or_else(|| DomainError::ValidationError(format!("{} tool not found: {}", expected_type, tool_id)))?;
        if tool.tool_type != expected_type {
            return Err(DomainError::ValidationError(format!("Tool {} is not a {} tool", tool_id, expected_type)));
        }
        if tool.tenant_id.is_some_and(|t| t != tenant_id) {
            return Err(DomainError::ValidationError(format!("Tool {} does not belong to this tenant", tool_id)));
        }
        Ok(())
    }
}

#[async_trait]
impl ProjectPort for ProjectService {
    async fn create_project(&self, command: CreateProjectCommand) -> Result<ProjectResponse, DomainError> {
        info!(slug = %command.slug, "Creating project");
        if command.name.trim().is_empty() {
            return Err(DomainError::ValidationError("Project name cannot be empty".into()));
        }
        if command.slug.trim().is_empty() {
            return Err(DomainError::ValidationError("Project slug cannot be empty".into()));
        }
        self.validate_tool_reference(command.github_tool_id, "github", command.tenant_id).await?;
        if let Some(jira_tool_id) = command.jira_tool_id {
            self.validate_tool_reference(jira_tool_id, "jira", command.tenant_id).await?;
        }
        if self.project_repository.find_by_slug_and_tenant(&command.slug, command.tenant_id).await?.is_some() {
            return Err(DomainError::AlreadyExists(format!("Project slug '{}' already exists in this tenant", command.slug)));
        }
        let default_branch = command.default_branch.filter(|b| !b.trim().is_empty()).unwrap_or_else(|| "main".to_string());
        let project = Project::new(NewProject {
            tenant_id: command.tenant_id,
            name: command.name,
            slug: command.slug,
            github_tool_id: command.github_tool_id,
            default_branch,
            jira_tool_id: command.jira_tool_id,
            build_command: command.build_command,
            test_command: command.test_command,
            coding_standards: command.coding_standards,
            workflow_config: command.workflow_config,
        });
        self.project_repository.save(&project).await?;
        info!(project_id = %project.id, "Project created");
        Ok(Self::to_response(&project))
    }

    async fn find_project_by_id(&self, id: Uuid) -> Result<ProjectResponse, DomainError> {
        let project = self.project_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Project not found: {}", id)))?;
        Ok(Self::to_response(&project))
    }

    async fn find_projects_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ProjectResponse>, DomainError> {
        let projects = self.project_repository.find_by_tenant(tenant_id).await?;
        Ok(projects.iter().map(Self::to_response).collect())
    }

    async fn update_project(&self, id: Uuid, command: UpdateProjectCommand) -> Result<ProjectResponse, DomainError> {
        info!(project_id = %id, "Updating project");
        let mut project = self.project_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Project not found: {}", id)))?;
        if let Some(name) = command.name {
            project.name = name;
        }
        if let Some(github_tool_id) = command.github_tool_id {
            self.validate_tool_reference(github_tool_id, "github", project.tenant_id).await?;
            project.github_tool_id = github_tool_id;
        }
        if let Some(default_branch) = command.default_branch {
            project.default_branch = default_branch;
        }
        if let Some(jira_tool_id) = command.jira_tool_id {
            self.validate_tool_reference(jira_tool_id, "jira", project.tenant_id).await?;
            project.jira_tool_id = Some(jira_tool_id);
        }
        if let Some(build_command) = command.build_command {
            project.build_command = Some(build_command);
        }
        if let Some(test_command) = command.test_command {
            project.test_command = Some(test_command);
        }
        if let Some(coding_standards) = command.coding_standards {
            project.coding_standards = Some(coding_standards);
        }
        if let Some(workflow_config) = command.workflow_config {
            project.workflow_config = Some(workflow_config);
        }
        if let Some(is_active) = command.is_active {
            project.is_active = is_active;
        }
        project.updated_at = Utc::now();
        self.project_repository.update(&project).await?;
        Ok(Self::to_response(&project))
    }

    async fn delete_project(&self, id: Uuid) -> Result<(), DomainError> {
        info!(project_id = %id, "Deleting project");
        let deleted = self.project_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(project_id = %id, "Project not found for deletion");
            return Err(DomainError::NotFound(format!("Project not found: {}", id)));
        }
        Ok(())
    }
}
