use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePromptCommand {
    pub tenant_id: Uuid,
    pub key: String,
    pub content: String,
    pub variables: Option<String>,
    pub description: Option<String>,
}
