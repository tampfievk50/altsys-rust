use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Scheduler {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub job_type: String, // "WEBHOOK" | "COMMAND"
    pub cron_expression: String,

    // WEBHOOK fields (job_type == WEBHOOK)
    pub webhook_url: Option<String>,
    pub http_method: Option<String>,
    pub headers: Option<Value>,
    pub body: Option<Value>,
    pub timeout_seconds: i32,

    // COMMAND fields (job_type == COMMAND)
    pub command_line: Option<String>,
    pub working_dir: Option<String>,
    pub detached: bool,
    pub last_pid: Option<i32>,

    pub is_active: bool,
    pub next_run_at: Option<DateTime<Utc>>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Scheduler {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        description: Option<String>,
        job_type: String,
        cron_expression: String,
        webhook_url: Option<String>,
        http_method: Option<String>,
        headers: Option<Value>,
        body: Option<Value>,
        timeout_seconds: i32,
        command_line: Option<String>,
        working_dir: Option<String>,
        detached: bool,
        next_run_at: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            job_type,
            cron_expression,
            webhook_url,
            http_method,
            headers,
            body,
            timeout_seconds,
            command_line,
            working_dir,
            detached,
            last_pid: None,
            is_active: true,
            next_run_at,
            last_run_at: None,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
