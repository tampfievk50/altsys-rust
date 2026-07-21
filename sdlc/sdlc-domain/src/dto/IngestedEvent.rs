use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct IngestedEvent {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub received_at: DateTime<Utc>,
}

pub struct NewIngestedEvent {
    pub tenant_id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
}

impl IngestedEvent {
    pub fn new(fields: NewIngestedEvent) -> Self {
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            event_type: fields.event_type,
            payload: fields.payload,
            received_at: Utc::now(),
        }
    }
}
