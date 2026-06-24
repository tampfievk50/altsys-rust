use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub action: String,
    pub resource: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Permission {
    pub fn new(name: String, action: String, resource: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            action,
            resource,
            description,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
