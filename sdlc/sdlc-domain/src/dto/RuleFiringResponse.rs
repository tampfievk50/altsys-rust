use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RuleFiringResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub rule_id: Uuid,
    pub matched: bool,
    pub status: String,
    pub action_result: Option<String>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
}
