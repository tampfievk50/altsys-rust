use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub slug: String,
    pub github_tool_id: Uuid,
    pub default_branch: String,
    pub jira_tool_id: Option<Uuid>,
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
