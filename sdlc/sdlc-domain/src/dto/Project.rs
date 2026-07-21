use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Project {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub slug: String,
    pub github_tool_id: Uuid,
    pub default_branch: String,
    pub jira_tool_id: Option<Uuid>,
    /// Set by `JiraPollingScheduler` after each successful poll (Phase 3 fallback
    /// ingestion path); `None` for projects that have never been polled.
    pub jira_last_synced_at: Option<DateTime<Utc>>,
    pub build_command: Option<String>,
    pub test_command: Option<String>,
    pub coding_standards: Option<String>,
    pub workflow_config: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewProject {
    pub tenant_id: Uuid,
    pub name: String,
    pub slug: String,
    pub github_tool_id: Uuid,
    pub default_branch: String,
    pub jira_tool_id: Option<Uuid>,
    pub build_command: Option<String>,
    pub test_command: Option<String>,
    pub coding_standards: Option<String>,
    pub workflow_config: Option<String>,
}

impl Project {
    pub fn new(fields: NewProject) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            name: fields.name,
            slug: fields.slug,
            github_tool_id: fields.github_tool_id,
            default_branch: fields.default_branch,
            jira_tool_id: fields.jira_tool_id,
            jira_last_synced_at: None,
            build_command: fields.build_command,
            test_command: fields.test_command,
            coding_standards: fields.coding_standards,
            workflow_config: fields.workflow_config,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
