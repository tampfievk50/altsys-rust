use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::CreateProjectCommand::CreateProjectCommand;
use sdlc_domain::dto::UpdateProjectCommand::UpdateProjectCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectRequest {
    pub tenant_id: Uuid,
    pub name: String,
    pub slug: String,
    pub github_tool_id: Uuid,
    pub default_branch: Option<String>,
    pub jira_tool_id: Option<Uuid>,
    pub build_command: Option<String>,
    pub test_command: Option<String>,
    pub coding_standards: Option<String>,
    pub workflow_config: Option<String>,
}

impl From<CreateProjectRequest> for CreateProjectCommand {
    fn from(val: CreateProjectRequest) -> Self {
        CreateProjectCommand {
            tenant_id: val.tenant_id,
            name: val.name,
            slug: val.slug,
            github_tool_id: val.github_tool_id,
            default_branch: val.default_branch,
            jira_tool_id: val.jira_tool_id,
            build_command: val.build_command,
            test_command: val.test_command,
            coding_standards: val.coding_standards,
            workflow_config: val.workflow_config,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub github_tool_id: Option<Uuid>,
    pub default_branch: Option<String>,
    pub jira_tool_id: Option<Uuid>,
    pub build_command: Option<String>,
    pub test_command: Option<String>,
    pub coding_standards: Option<String>,
    pub workflow_config: Option<String>,
    pub is_active: Option<bool>,
}

impl From<UpdateProjectRequest> for UpdateProjectCommand {
    fn from(val: UpdateProjectRequest) -> Self {
        UpdateProjectCommand {
            name: val.name,
            github_tool_id: val.github_tool_id,
            default_branch: val.default_branch,
            jira_tool_id: val.jira_tool_id,
            build_command: val.build_command,
            test_command: val.test_command,
            coding_standards: val.coding_standards,
            workflow_config: val.workflow_config,
            is_active: val.is_active,
        }
    }
}
