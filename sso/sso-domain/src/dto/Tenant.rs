use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Tenant {
    pub fn new(name: String, slug: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            slug,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
