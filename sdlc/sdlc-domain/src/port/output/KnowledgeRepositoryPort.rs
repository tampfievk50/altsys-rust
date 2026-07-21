use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::KnowledgeItem::KnowledgeItem;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait KnowledgeRepositoryPort: Send + Sync {
    async fn save(&self, item: &KnowledgeItem) -> Result<(), DomainError>;
    async fn update(&self, item: &KnowledgeItem) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<KnowledgeItem>, DomainError>;
    /// All versions of one key, scoped to the tenant.
    async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<KnowledgeItem>, DomainError>;
    /// All versions of every key, scoped to the tenant.
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<KnowledgeItem>, DomainError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError>;

    /// Ranks the latest, active version of every key by vector similarity to
    /// `query_embedding` (highest similarity first), optionally restricted to one
    /// `source_type`, capped at `limit` rows. Backed by a native vector index
    /// (pgvector) in the dataaccess layer — the ranking itself runs in the database,
    /// not in application code.
    async fn find_nearest_by_tenant(
        &self,
        tenant_id: Uuid,
        query_embedding: &[f32],
        source_type: Option<&str>,
        limit: u64,
    ) -> Result<Vec<(KnowledgeItem, f32)>, DomainError>;
}
