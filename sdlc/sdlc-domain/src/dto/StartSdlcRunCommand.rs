use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The IDs an SDLC run wires together, resolved by the caller from the four
/// services this orchestrator drives (Platform, Tools, Knowledge, Agents). GitHub
/// and Jira tools are optional: omitting them runs the pipeline as a local dry run
/// that skips `CreatePullRequest` / `UpdateJira`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartSdlcRunCommand {
    pub tenant_id: Uuid,
    pub project_id: Uuid,
    pub ticket_key: String,
    /// Manually supplied ticket description, used when `jira_tool_id` is absent
    /// (or as a fallback if the Jira fetch fails to return a summary).
    pub ticket_summary: Option<String>,
    pub planner_agent_id: Uuid,
    pub architect_agent_id: Uuid,
    pub developer_agent_id: Uuid,
    pub reviewer_agent_id: Uuid,
    pub documentation_agent_id: Uuid,
    pub git_tool_id: Uuid,
    pub build_tool_id: Uuid,
    /// The Filesystem tool (`tool_type: "filesystem"`) whose `working_directory`
    /// must match `git_tool_id`'s checkout — the Developer step writes the
    /// agent's file edits through this tool.
    pub filesystem_tool_id: Uuid,
    pub github_tool_id: Option<Uuid>,
    pub jira_tool_id: Option<Uuid>,
}
