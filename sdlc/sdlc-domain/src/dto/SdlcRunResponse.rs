use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SdlcRunResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub project_id: Uuid,
    pub ticket_key: String,
    pub status: String,
    pub current_step: Option<String>,
    pub branch_name: Option<String>,
    pub pull_request_url: Option<String>,
    pub context: String,
    pub error: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
