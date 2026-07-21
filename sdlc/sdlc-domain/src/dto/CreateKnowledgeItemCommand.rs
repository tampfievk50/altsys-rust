use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKnowledgeItemCommand {
    pub tenant_id: Uuid,
    pub source_type: String,
    pub key: String,
    pub title: String,
    pub content: String,
    pub metadata: Option<String>,
}
