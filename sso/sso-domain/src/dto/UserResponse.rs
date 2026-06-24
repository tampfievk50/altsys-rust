use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}
