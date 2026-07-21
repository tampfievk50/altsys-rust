use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateKnowledgeItemCommand {
    pub title: Option<String>,
    /// Updating content re-runs the embedding pipeline for this item.
    pub content: Option<String>,
    pub metadata: Option<String>,
    pub is_active: Option<bool>,
}
