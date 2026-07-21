use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Model {
    pub id: Uuid,
    /// `None` marks a global/platform-wide model available to all tenants.
    pub tenant_id: Option<Uuid>,
    pub provider: String,
    pub model_name: String,
    pub capability: String,
    pub credential_id: Option<Uuid>,
    pub endpoint_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewModel {
    pub tenant_id: Option<Uuid>,
    pub provider: String,
    pub model_name: String,
    pub capability: String,
    pub credential_id: Option<Uuid>,
    pub endpoint_url: Option<String>,
}

impl Model {
    pub fn new(fields: NewModel) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            provider: fields.provider,
            model_name: fields.model_name,
            capability: fields.capability,
            credential_id: fields.credential_id,
            endpoint_url: fields.endpoint_url,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
