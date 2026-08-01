use serde::{Deserialize, Serialize};
use serde_json::Value;

// job_type cannot be changed via update: switching between WEBHOOK and COMMAND
// changes which fields are meaningful, so a scheduler that needs a different
// job_type should be deleted and recreated instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSchedulerCommand {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cron_expression: Option<String>,
    pub timeout_seconds: Option<i32>,

    pub webhook_url: Option<String>,
    pub http_method: Option<String>,
    pub headers: Option<Value>,
    pub body: Option<Value>,

    pub command_line: Option<String>,
    pub working_dir: Option<String>,
    pub detached: Option<bool>,
}
