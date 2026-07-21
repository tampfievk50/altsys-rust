use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::KnowledgeSearchResult::KnowledgeSearchResult;
use crate::dto::SemanticSearchCommand::SemanticSearchCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait SemanticSearchPort: Send + Sync {
    /// Embeds `command.query`, then ranks the tenant's latest-version knowledge items by
    /// cosine similarity against their stored embeddings.
    async fn search(&self, tenant_id: Uuid, command: SemanticSearchCommand) -> Result<Vec<KnowledgeSearchResult>, DomainError>;
}
