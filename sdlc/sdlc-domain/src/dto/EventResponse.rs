use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub event_type: String,
    pub payload: String,
    pub received_at: DateTime<Utc>,
}
