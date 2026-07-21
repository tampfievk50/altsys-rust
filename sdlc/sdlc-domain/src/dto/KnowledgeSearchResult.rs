use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dto::KnowledgeItemResponse::KnowledgeItemResponse;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct KnowledgeSearchResult {
    pub item: KnowledgeItemResponse,
    /// Cosine similarity between the query embedding and the item's embedding, in `[-1.0, 1.0]`.
    pub score: f32,
}
