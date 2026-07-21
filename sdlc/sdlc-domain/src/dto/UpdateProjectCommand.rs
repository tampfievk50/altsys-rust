use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProjectCommand {
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
