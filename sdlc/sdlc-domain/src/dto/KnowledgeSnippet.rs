use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSnippet {
    pub title: String,
    pub content: String,
    pub score: f32,
}
