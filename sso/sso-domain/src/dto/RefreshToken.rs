use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
    pub created_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn new(user_id: Uuid, tenant_id: Uuid, token: String, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            tenant_id,
            token,
            expires_at,
            is_revoked: false,
            created_at: Utc::now(),
        }
    }
}
