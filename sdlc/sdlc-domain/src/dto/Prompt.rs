use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Prompt {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub key: String,
    pub version: i32,
    pub content: String,
    pub variables: Option<String>,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewPrompt {
    pub tenant_id: Uuid,
    pub key: String,
    pub version: i32,
    pub content: String,
    pub variables: Option<String>,
    pub description: Option<String>,
}

impl Prompt {
    pub fn new(fields: NewPrompt) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            key: fields.key,
            version: fields.version,
            content: fields.content,
            variables: fields.variables,
            description: fields.description,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
