use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCredentialCommand {
    pub tenant_id: Uuid,
    pub name: String,
    pub provider: String,
    pub secret: String,
    pub metadata: Option<String>,
}
