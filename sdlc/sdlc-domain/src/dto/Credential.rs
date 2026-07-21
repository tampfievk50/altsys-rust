use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Credential {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub provider: String,
    pub encrypted_secret: String,
    pub secret_hint: Option<String>,
    pub metadata: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewCredential {
    pub tenant_id: Uuid,
    pub name: String,
    pub provider: String,
    pub encrypted_secret: String,
    pub secret_hint: Option<String>,
    pub metadata: Option<String>,
}

impl Credential {
    pub fn new(fields: NewCredential) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            name: fields.name,
            provider: fields.provider,
            encrypted_secret: fields.encrypted_secret,
            secret_hint: fields.secret_hint,
            metadata: fields.metadata,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
