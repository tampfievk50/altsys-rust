use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PermissionResponse {
    pub id: Uuid,
    pub name: String,
    pub action: String,
    pub resource: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
