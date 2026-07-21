use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use sdlc_domain::dto::StartSdlcRunCommand::StartSdlcRunCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub struct StartSdlcRunRequest {
    pub tenant_id: Uuid,
    pub project_id: Uuid,
    pub ticket_key: String,
    pub ticket_summary: Option<String>,
    pub planner_agent_id: Uuid,
    pub architect_agent_id: Uuid,
    pub developer_agent_id: Uuid,
    pub reviewer_agent_id: Uuid,
    pub documentation_agent_id: Uuid,
    pub git_tool_id: Uuid,
    pub build_tool_id: Uuid,
    pub filesystem_tool_id: Uuid,
    pub github_tool_id: Option<Uuid>,
    pub jira_tool_id: Option<Uuid>,
}

impl From<StartSdlcRunRequest> for StartSdlcRunCommand {
    fn from(val: StartSdlcRunRequest) -> Self {
        StartSdlcRunCommand {
            tenant_id: val.tenant_id,
            project_id: val.project_id,
            ticket_key: val.ticket_key,
            ticket_summary: val.ticket_summary,
            planner_agent_id: val.planner_agent_id,
            architect_agent_id: val.architect_agent_id,
            developer_agent_id: val.developer_agent_id,
            reviewer_agent_id: val.reviewer_agent_id,
            documentation_agent_id: val.documentation_agent_id,
            git_tool_id: val.git_tool_id,
            build_tool_id: val.build_tool_id,
            filesystem_tool_id: val.filesystem_tool_id,
            github_tool_id: val.github_tool_id,
            jira_tool_id: val.jira_tool_id,
        }
    }
}
