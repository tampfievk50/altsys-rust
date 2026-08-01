use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SchedulerResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub job_type: String,
    pub cron_expression: String,
    pub webhook_url: Option<String>,
    pub http_method: Option<String>,
    pub headers: Option<Value>,
    pub body: Option<Value>,
    pub timeout_seconds: i32,
    pub command_line: Option<String>,
    pub working_dir: Option<String>,
    pub detached: bool,
    pub last_pid: Option<i32>,
    pub is_active: bool,
    pub next_run_at: Option<DateTime<Utc>>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
