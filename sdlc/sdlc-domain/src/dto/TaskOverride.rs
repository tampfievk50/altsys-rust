use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A user-edited field for a Jira-ingested "task" (ticket), stored alongside
/// the write-through call to Jira itself — Jira stays the source of truth,
/// but the admin panel doesn't have to wait for the next ingested event to
/// show the edit back.
#[derive(Debug, Clone)]
pub struct TaskOverride {
    pub id: Uuid,
    pub project_id: Uuid,
    pub ticket_key: String,
    pub summary: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewTaskOverride {
    pub project_id: Uuid,
    pub ticket_key: String,
    pub summary: String,
}

impl TaskOverride {
    pub fn new(fields: NewTaskOverride) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            project_id: fields.project_id,
            ticket_key: fields.ticket_key,
            summary: fields.summary,
            created_at: now,
            updated_at: now,
        }
    }
}
