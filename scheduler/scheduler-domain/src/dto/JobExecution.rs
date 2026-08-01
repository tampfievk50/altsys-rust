use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct JobExecution {
    pub id: Uuid,
    pub scheduler_id: Uuid,
    pub trigger_type: String,
    pub status: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status_code: Option<i32>,
    pub response_body: Option<String>,
    pub error_message: Option<String>,
}

impl JobExecution {
    pub fn start(scheduler_id: Uuid, trigger_type: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            scheduler_id,
            trigger_type: trigger_type.to_string(),
            status: "RUNNING".to_string(),
            started_at: Utc::now(),
            finished_at: None,
            status_code: None,
            response_body: None,
            error_message: None,
        }
    }
}
