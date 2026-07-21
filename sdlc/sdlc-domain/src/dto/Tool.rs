use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Tool {
    pub id: Uuid,
    /// `None` marks a global/platform-wide tool available to all tenants.
    pub tenant_id: Option<Uuid>,
    pub name: String,
    /// One of the built-in executor types: `git`, `github`, `jira`, `filesystem`, `cargo`, `maven`, `gradle`.
    pub tool_type: String,
    pub description: Option<String>,
    /// Free-form JSON configuration interpreted by the matching `ToolExecutorPort` (e.g. repo path, base URL).
    pub config: Option<String>,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewTool {
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub tool_type: String,
    pub description: Option<String>,
    pub config: Option<String>,
}

impl Tool {
    pub fn new(fields: NewTool) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            name: fields.name,
            tool_type: fields.tool_type,
            description: fields.description,
            config: fields.config,
            is_enabled: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
