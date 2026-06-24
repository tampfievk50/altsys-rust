use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleCommand {
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
