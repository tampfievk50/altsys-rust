use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePluginCommand {
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub webhook_url: String,
    pub secret: Option<String>,
}
