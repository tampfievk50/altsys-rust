use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::KnowledgeSnippet::KnowledgeSnippet;
use crate::r#enum::DomainError::DomainError;

/// Driven adapter for the Knowledge service (Phase 3): semantic search over
/// organizational knowledge to ground the Planner Agent's prompt.
#[async_trait]
pub trait KnowledgeClientPort: Send + Sync {
    async fn search(&self, tenant_id: Uuid, query: String, limit: u32) -> Result<Vec<KnowledgeSnippet>, DomainError>;
}
