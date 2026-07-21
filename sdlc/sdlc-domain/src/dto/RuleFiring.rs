use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::dto::RuleFiringStatus::RuleFiringStatus;

#[derive(Debug, Clone)]
pub struct RuleFiring {
    pub id: Uuid,
    pub event_id: Uuid,
    pub rule_id: Uuid,
    pub matched: bool,
    pub status: RuleFiringStatus,
    pub action_result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct NewRuleFiring {
    pub event_id: Uuid,
    pub rule_id: Uuid,
    pub matched: bool,
    pub status: RuleFiringStatus,
    pub action_result: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl RuleFiring {
    pub fn new(fields: NewRuleFiring) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_id: fields.event_id,
            rule_id: fields.rule_id,
            matched: fields.matched,
            status: fields.status,
            action_result: fields.action_result,
            error: fields.error,
            created_at: Utc::now(),
        }
    }
}
