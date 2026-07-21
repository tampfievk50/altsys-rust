use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::dto::SdlcRunStatus::SdlcRunStatus;

#[derive(Debug, Clone)]
pub struct SdlcRun {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub project_id: Uuid,
    pub ticket_key: String,
    pub status: SdlcRunStatus,
    pub current_step: Option<String>,
    pub branch_name: Option<String>,
    pub pull_request_url: Option<String>,
    /// Accumulated JSON output of every completed step, keyed by step name
    /// (e.g. `context["compile"]`) — mirrors the Workflow service's execution context.
    pub context: serde_json::Value,
    pub error: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewSdlcRun {
    pub tenant_id: Uuid,
    pub project_id: Uuid,
    pub ticket_key: String,
}

impl SdlcRun {
    pub fn new(fields: NewSdlcRun) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            project_id: fields.project_id,
            ticket_key: fields.ticket_key,
            status: SdlcRunStatus::Running,
            current_step: None,
            branch_name: None,
            pull_request_url: None,
            context: serde_json::json!({}),
            error: None,
            started_at: Some(now),
            completed_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Shallow-merges a step's JSON output into the run context under `key`.
    pub fn merge_context(&mut self, key: &str, value: &serde_json::Value) {
        if let Some(base) = self.context.as_object_mut() {
            base.insert(key.to_string(), value.clone());
        }
    }
}
