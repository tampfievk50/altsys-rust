use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSchedulerCommand {
    pub name: String,
    pub description: Option<String>,
    pub job_type: String, // "WEBHOOK" | "COMMAND"
    pub cron_expression: String,

    // Required when job_type == WEBHOOK
    pub webhook_url: Option<String>,
    pub http_method: Option<String>,
    pub headers: Option<Value>,
    pub body: Option<Value>,
    pub timeout_seconds: Option<i32>,

    // Required when job_type == COMMAND
    pub command_line: Option<String>,
    pub working_dir: Option<String>,
    pub detached: Option<bool>,
}
