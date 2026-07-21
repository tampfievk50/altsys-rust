use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use sdlc_domain::dto::KnowledgeItem::KnowledgeItem;
use sdlc_domain::port::output::KnowledgeRepositoryPort::KnowledgeRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;

use crate::knowledge_item::mapper::KnowledgeItemDataMapper::KnowledgeItemDataMapper;
use crate::knowledge_item::repository::KnowledgeItemSeaOrmRepository::KnowledgeItemSeaOrmRepository;

pub struct KnowledgeItemRepositoryImpl {
    sea_orm_repo: KnowledgeItemSeaOrmRepository,
}

impl KnowledgeItemRepositoryImpl {
    pub fn new(sea_orm_repo: KnowledgeItemSeaOrmRepository) -> Self {
        Self { sea_orm_repo }
    }
}

#[async_trait]
impl KnowledgeRepositoryPort for KnowledgeItemRepositoryImpl {
    async fn save(&self, item: &KnowledgeItem) -> Result<(), DomainError> {
        self.sea_orm_repo.insert(KnowledgeItemDataMapper::to_active_model(item)).await
            .map_err(|e| { error!(error = %e, "Failed to save knowledge item"); DomainError::InternalError(e.to_string()) })?;
        if let Some(embedding) = &item.embedding {
            self.sea_orm_repo.set_embedding(item.id, embedding).await
                .map_err(|e| { error!(error = %e, "Failed to save knowledge item embedding"); DomainError::InternalError(e.to_string()) })?;
        }
        Ok(())
    }

    async fn update(&self, item: &KnowledgeItem) -> Result<(), DomainError> {
        self.sea_orm_repo.update(KnowledgeItemDataMapper::to_active_model(item)).await
            .map_err(|e| { error!(error = %e, "Failed to update knowledge item"); DomainError::InternalError(e.to_string()) })?;
        if let Some(embedding) = &item.embedding {
            self.sea_orm_repo.set_embedding(item.id, embedding).await
                .map_err(|e| { error!(error = %e, "Failed to update knowledge item embedding"); DomainError::InternalError(e.to_string()) })?;
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<KnowledgeItem>, DomainError> {
        self.sea_orm_repo.find_by_id(id).await
            .map(|opt| opt.as_ref().map(KnowledgeItemDataMapper::to_domain))
            .map_err(|e| { error!(error = %e, "Failed to find knowledge item"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<KnowledgeItem>, DomainError> {
        self.sea_orm_repo.find_all_by_key_and_tenant(tenant_id, key).await
            .map(|rows| rows.iter().map(KnowledgeItemDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list knowledge item versions"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<KnowledgeItem>, DomainError> {
        self.sea_orm_repo.find_by_tenant(tenant_id).await
            .map(|rows| rows.iter().map(KnowledgeItemDataMapper::to_domain).collect())
            .map_err(|e| { error!(error = %e, "Failed to list knowledge items"); DomainError::InternalError(e.to_string()) })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        self.sea_orm_repo.delete_by_id(id).await
            .map_err(|e| { error!(error = %e, "Failed to delete knowledge item"); DomainError::InternalError(e.to_string()) })
    }

    async fn find_nearest_by_tenant(
        &self,
        tenant_id: Uuid,
        query_embedding: &[f32],
        source_type: Option<&str>,
        limit: u64,
    ) -> Result<Vec<(KnowledgeItem, f32)>, DomainError> {
        self.sea_orm_repo.find_nearest(tenant_id, query_embedding, source_type, limit).await
            .map(|rows| rows.into_iter().map(|row| {
                let item = KnowledgeItem {
                    id: row.id,
                    tenant_id: row.tenant_id,
                    source_type: row.source_type,
                    key: row.key,
                    version: row.version,
                    title: row.title,
                    content: row.content,
                    metadata: row.metadata,
                    // Not selected by the nearest-neighbor query (the caller already
                    // supplied the query vector; echoing the stored one back is redundant).
                    embedding: None,
                    is_active: row.is_active,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    created_by: row.created_by,
                    updated_by: row.updated_by,
                };
                // pgvector's `<=>` is cosine distance (0 = identical, 2 = opposite); convert to similarity.
                let similarity = 1.0 - row.distance as f32;
                (item, similarity)
            }).collect())
            .map_err(|e| { error!(error = %e, "Failed to run semantic search"); DomainError::InternalError(e.to_string()) })
    }
}
