use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;

use crate::dto::KnowledgeItem::KnowledgeItem;
use crate::dto::KnowledgeItemResponse::KnowledgeItemResponse;
use crate::dto::KnowledgeSearchResult::KnowledgeSearchResult;
use crate::dto::SemanticSearchCommand::SemanticSearchCommand;
use crate::port::input::SemanticSearchPort::SemanticSearchPort;
use crate::port::output::EmbeddingProviderPort::EmbeddingProviderPort;
use crate::port::output::KnowledgeRepositoryPort::KnowledgeRepositoryPort;
use crate::r#enum::DomainError::DomainError;

const DEFAULT_LIMIT: u64 = 10;
const MAX_LIMIT: u64 = 50;

pub struct KnowledgeSearchService {
    knowledge_repository: Arc<dyn KnowledgeRepositoryPort>,
    embedding_provider: Arc<dyn EmbeddingProviderPort>,
}

impl KnowledgeSearchService {
    pub fn new(knowledge_repository: Arc<dyn KnowledgeRepositoryPort>, embedding_provider: Arc<dyn EmbeddingProviderPort>) -> Self {
        Self { knowledge_repository, embedding_provider }
    }

    fn to_response(item: &KnowledgeItem) -> KnowledgeItemResponse {
        KnowledgeItemResponse {
            id: item.id,
            tenant_id: item.tenant_id,
            source_type: item.source_type.clone(),
            key: item.key.clone(),
            version: item.version,
            title: item.title.clone(),
            content: item.content.clone(),
            metadata: item.metadata.clone(),
            embedding: item.embedding.clone(),
            is_active: item.is_active,
            created_at: item.created_at,
            updated_at: item.updated_at,
            created_by: item.created_by,
            updated_by: item.updated_by,
        }
    }
}

#[async_trait]
impl SemanticSearchPort for KnowledgeSearchService {
    async fn search(&self, tenant_id: Uuid, command: SemanticSearchCommand) -> Result<Vec<KnowledgeSearchResult>, DomainError> {
        if command.query.trim().is_empty() {
            return Err(DomainError::ValidationError("Query cannot be empty".into()));
        }
        let limit = command.limit.map(u64::from).unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);

        let query_embedding = self.embedding_provider.embed(&command.query).await?;
        let ranked = self.knowledge_repository
            .find_nearest_by_tenant(tenant_id, &query_embedding, command.source_type.as_deref(), limit)
            .await?;

        info!(tenant_id = %tenant_id, results = ranked.len(), "Semantic search completed");
        Ok(ranked.into_iter().map(|(item, score)| KnowledgeSearchResult { item: Self::to_response(&item), score }).collect())
    }
}
