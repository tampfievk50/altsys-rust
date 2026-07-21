use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use sdlc_domain::dto::KnowledgeSnippet::KnowledgeSnippet;
use sdlc_domain::dto::SemanticSearchCommand::SemanticSearchCommand;
use sdlc_domain::port::input::SemanticSearchPort::SemanticSearchPort;
use sdlc_domain::port::output::KnowledgeClientPort::KnowledgeClientPort;
use sdlc_domain::r#enum::DomainError::DomainError;

/// Replaces the old HTTP call to the Knowledge service with a direct call to
/// the merged `KnowledgeSearchService`.
pub struct InProcessKnowledgeClient {
    semantic_search_port: Arc<dyn SemanticSearchPort>,
}

impl InProcessKnowledgeClient {
    pub fn new(semantic_search_port: Arc<dyn SemanticSearchPort>) -> Self {
        Self { semantic_search_port }
    }
}

#[async_trait]
impl KnowledgeClientPort for InProcessKnowledgeClient {
    async fn search(&self, tenant_id: Uuid, query: String, limit: u32) -> Result<Vec<KnowledgeSnippet>, DomainError> {
        let results = self.semantic_search_port
            .search(tenant_id, SemanticSearchCommand { query, source_type: None, limit: Some(limit) })
            .await?;
        Ok(results.into_iter()
            .map(|r| KnowledgeSnippet { title: r.item.title, content: r.item.content, score: r.score })
            .collect())
    }
}
