use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSkillCommand {
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub content: String,
}
