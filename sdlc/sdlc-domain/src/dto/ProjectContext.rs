use serde::{Deserialize, Serialize};

/// The subset of the Platform service's `Project` this orchestrator needs. Extra
/// fields on the Platform response (id, name, audit fields, ...) are ignored by
/// serde on deserialize — the two services stay independently versioned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    pub repository_url: String,
    pub default_branch: String,
    pub jira_project_key: Option<String>,
    pub build_command: Option<String>,
    pub test_command: Option<String>,
}
