use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CredentialResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub provider: String,
    pub secret_hint: Option<String>,
    pub metadata: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    // encrypted_secret is intentionally ABSENT
}
