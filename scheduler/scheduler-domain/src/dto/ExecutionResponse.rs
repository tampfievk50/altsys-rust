use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExecutionResponse {
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
