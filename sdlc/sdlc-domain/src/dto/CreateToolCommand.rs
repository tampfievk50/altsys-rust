use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateToolCommand {
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub tool_type: String,
    pub description: Option<String>,
    pub config: Option<String>,
}
