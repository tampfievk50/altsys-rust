use async_trait::async_trait;
use uuid::Uuid;

use crate::dto::CreateKnowledgeItemCommand::CreateKnowledgeItemCommand;
use crate::dto::KnowledgeItemResponse::KnowledgeItemResponse;
use crate::dto::UpdateKnowledgeItemCommand::UpdateKnowledgeItemCommand;
use crate::r#enum::DomainError::DomainError;

#[async_trait]
pub trait KnowledgePort: Send + Sync {
    /// Creates a new knowledge item version for `command.key`; the version number is
    /// computed automatically (max existing version for the key, tenant + 1) and the
    /// content is embedded via the configured `EmbeddingProviderPort`.
    async fn create_knowledge_item(&self, command: CreateKnowledgeItemCommand) -> Result<KnowledgeItemResponse, DomainError>;
    async fn find_knowledge_item_by_id(&self, id: Uuid) -> Result<KnowledgeItemResponse, DomainError>;
    async fn find_latest_knowledge_item_by_key(&self, tenant_id: Uuid, key: &str) -> Result<KnowledgeItemResponse, DomainError>;
    async fn find_knowledge_item_versions_by_key(&self, tenant_id: Uuid, key: &str) -> Result<Vec<KnowledgeItemResponse>, DomainError>;
    /// Returns the latest version of every distinct key for the tenant.
    async fn find_knowledge_items_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<KnowledgeItemResponse>, DomainError>;
    /// Updates the item in place; re-embeds content when it changes. Does not create a new version.
    async fn update_knowledge_item(&self, id: Uuid, command: UpdateKnowledgeItemCommand) -> Result<KnowledgeItemResponse, DomainError>;
    async fn delete_knowledge_item(&self, id: Uuid) -> Result<(), DomainError>;
}
