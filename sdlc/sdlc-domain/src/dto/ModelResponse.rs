use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModelResponse {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub provider: String,
    pub model_name: String,
    pub capability: String,
    pub credential_id: Option<Uuid>,
    pub endpoint_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}
