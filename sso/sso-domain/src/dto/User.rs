use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl User {
    pub fn new(
        tenant_id: Uuid,
        username: String,
        email: String,
        password_hash: String,
        full_name: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            username,
            email,
            password_hash,
            full_name,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
