use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use sdlc_domain::dto::ProjectContext::ProjectContext;
use sdlc_domain::port::input::ProjectPort::ProjectPort;
use sdlc_domain::port::output::PlatformClientPort::PlatformClientPort;
use sdlc_domain::port::output::ToolRepositoryPort::ToolRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Replaces the old HTTP call to the Platform service with a direct call to
/// the merged `ProjectService`, now that both live in the same process.
pub struct InProcessPlatformClient {
    project_port: Arc<dyn ProjectPort>,
    tool_repository: Arc<dyn ToolRepositoryPort>,
}

impl InProcessPlatformClient {
    pub fn new(project_port: Arc<dyn ProjectPort>, tool_repository: Arc<dyn ToolRepositoryPort>) -> Self {
        Self { project_port, tool_repository }
    }

    /// A Project only stores *which* Tool to use; the actual repository slug
    /// or Jira project key lives in that Tool's opaque `config` JSON (set up
    /// once in Settings → GitHub/Jira, reused by every project that selects it).
    fn config_field(config: Option<&str>, field: &str) -> Option<String> {
        let value: serde_json::Value = serde_json::from_str(config?).ok()?;
        value.get(field)?.as_str().map(str::to_string)
    }
}

#[async_trait]
impl PlatformClientPort for InProcessPlatformClient {
    async fn get_project(&self, project_id: Uuid) -> Result<ProjectContext, DomainError> {
        let project = self.project_port.find_project_by_id(project_id).await?;

        let github_tool = self.tool_repository.find_by_id(project.github_tool_id).await?
            .ok_or_else(|| DomainError::InternalError(format!("GitHub tool not found: {}", project.github_tool_id)))?;
        let repository_url = Self::config_field(github_tool.config.as_deref(), "repository")
            .ok_or_else(|| DomainError::InternalError(format!("GitHub tool {} has no 'repository' in its config", project.github_tool_id)))?;

        let jira_project_key = match project.jira_tool_id {
            Some(jira_tool_id) => {
                let jira_tool = self.tool_repository.find_by_id(jira_tool_id).await?
                    .ok_or_else(|| DomainError::InternalError(format!("Jira tool not found: {}", jira_tool_id)))?;
                Self::config_field(jira_tool.config.as_deref(), "project_key")
            }
            None => None,
        };

        Ok(ProjectContext {
            repository_url,
            default_branch: project.default_branch,
            jira_project_key,
            build_command: project.build_command,
            test_command: project.test_command,
        })
    }
}
