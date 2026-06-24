use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Role {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Role {
    pub fn new(tenant_id: Uuid, name: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            name,
            description,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
