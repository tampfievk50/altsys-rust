use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A third-party extension point (the "Plugin SDK"): a registered webhook that the
/// automation engine invokes when an `AutomationRule`'s action is `invoke_plugin`.
#[derive(Debug, Clone)]
pub struct Plugin {
    pub id: Uuid,
    /// `None` marks a global/platform-wide plugin available to all tenants.
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub webhook_url: String,
    /// Sent back as `X-Automation-Secret` on every dispatch so the plugin can verify
    /// the call originated from this engine.
    pub secret: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewPlugin {
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub webhook_url: String,
    pub secret: Option<String>,
}

impl Plugin {
    pub fn new(fields: NewPlugin) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            name: fields.name,
            webhook_url: fields.webhook_url,
            secret: fields.secret,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
