use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SdlcStepExecutionResponse {
    pub id: Uuid,
    pub run_id: Uuid,
    pub step: String,
    pub attempt: i32,
    pub status: String,
    pub output: Option<String>,
    pub error: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}
