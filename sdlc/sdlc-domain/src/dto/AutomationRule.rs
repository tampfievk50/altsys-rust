use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AutomationRule {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    /// e.g. `jira.ticket.transitioned`, `github.pull_request.opened`, `manual`.
    pub event_type: String,
    /// JSON object of exact-match `field: value` pairs evaluated against the
    /// ingested event's payload; `None` matches every event of this `event_type`.
    pub match_criteria: Option<String>,
    /// Raw JSON text of an `ActionSpec`, parsed at dispatch time.
    pub action: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewAutomationRule {
    pub tenant_id: Uuid,
    pub name: String,
    pub event_type: String,
    pub match_criteria: Option<String>,
    pub action: String,
}

impl AutomationRule {
    pub fn new(fields: NewAutomationRule) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            name: fields.name,
            event_type: fields.event_type,
            match_criteria: fields.match_criteria,
            action: fields.action,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }

    /// All criteria must equal the corresponding payload field (AND semantics); a
    /// missing `match_criteria` matches unconditionally. Flat, top-level fields only.
    pub fn matches(&self, payload: &serde_json::Value) -> bool {
        let Some(raw) = &self.match_criteria else { return true };
        let Ok(serde_json::Value::Object(criteria)) = serde_json::from_str::<serde_json::Value>(raw) else { return false };
        criteria.iter().all(|(key, expected)| payload.get(key) == Some(expected))
    }
}
