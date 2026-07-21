use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;

use crate::dto::CreateKnowledgeItemCommand::CreateKnowledgeItemCommand;
use crate::dto::KnowledgeItem::{KnowledgeItem, NewKnowledgeItem};
use crate::dto::KnowledgeItemResponse::KnowledgeItemResponse;
use crate::dto::UpdateKnowledgeItemCommand::UpdateKnowledgeItemCommand;
use crate::port::input::KnowledgePort::KnowledgePort;
use crate::port::output::EmbeddingProviderPort::EmbeddingProviderPort;
use crate::port::output::KnowledgeRepositoryPort::KnowledgeRepositoryPort;
use crate::r#enum::DomainError::DomainError;

pub struct KnowledgeService {
    knowledge_repository: Arc<dyn KnowledgeRepositoryPort>,
    embedding_provider: Arc<dyn EmbeddingProviderPort>,
}

impl KnowledgeService {
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

    async fn embed(&self, text: &str) -> Result<Vec<f32>, DomainError> {
        self.embedding_provider.embed(text).await
    }
}

#[async_trait]
impl KnowledgePort for KnowledgeService {
    async fn create_knowledge_item(&self, command: CreateKnowledgeItemCommand) -> Result<KnowledgeItemResponse, DomainError> {
        info!(key = %command.key, source_type = %command.source_type, "Creating knowledge item version");
        if command.source_type.trim().is_empty() {
            return Err(DomainError::ValidationError("Source type cannot be empty".into()));
        }
        if command.key.trim().is_empty() {
            return Err(DomainError::ValidationError("Key cannot be empty".into()));
        }
        if command.title.trim().is_empty() {
            return Err(DomainError::ValidationError("Title cannot be empty".into()));
        }
        if command.content.trim().is_empty() {
            return Err(DomainError::ValidationError("Content cannot be empty".into()));
        }

        let existing_versions = self.knowledge_repository.find_all_by_key_and_tenant(command.tenant_id, &command.key).await?;
        let next_version = existing_versions.iter().map(|i| i.version).max().unwrap_or(0) + 1;
        let embedding = self.embed(&command.content).await?;

        let item = KnowledgeItem::new(NewKnowledgeItem {
            tenant_id: command.tenant_id,
            source_type: command.source_type,
            key: command.key,
            version: next_version,
            title: command.title,
            content: command.content,
            metadata: command.metadata,
            embedding: Some(embedding),
        });
        self.knowledge_repository.save(&item).await?;
        info!(item_id = %item.id, version = item.version, "Knowledge item version created");
        Ok(Self::to_response(&item))
    }

    async fn find_knowledge_item_by_id(&self, id: Uuid) -> Result<KnowledgeItemResponse, DomainError> {
        let item = self.knowledge_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Knowledge item not found: {}", id)))?;
        Ok(Self::to_response(&item))
    }

    async fn find_latest_knowledge_item_by_key(&self, tenant_id: Uuid, key: &str) -> Result<KnowledgeItemResponse, DomainError> {
        let versions = self.knowledge_repository.find_all_by_key_and_tenant(tenant_id, key).await?;
        let latest = versions.into_iter().max_by_key(|i| i.version)
            .ok_or_else(|| DomainError::NotFound(format!("Knowledge item not found for key: {}", key)))?;
        Ok(Self::to_response(&latest))
    }

    async fn find_knowledge_item_versions_by_key(&self, tenant_id: Uuid, key: &str) -> Result<Vec<KnowledgeItemResponse>, DomainError> {
        let mut versions = self.knowledge_repository.find_all_by_key_and_tenant(tenant_id, key).await?;
        versions.sort_by_key(|i| i.version);
        Ok(versions.iter().map(Self::to_response).collect())
    }

    async fn find_knowledge_items_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<KnowledgeItemResponse>, DomainError> {
        let items = self.knowledge_repository.find_by_tenant(tenant_id).await?;
        Ok(KnowledgeItem::latest_per_key(items).iter().map(Self::to_response).collect())
    }

    async fn update_knowledge_item(&self, id: Uuid, command: UpdateKnowledgeItemCommand) -> Result<KnowledgeItemResponse, DomainError> {
        info!(item_id = %id, "Updating knowledge item");
        let mut item = self.knowledge_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Knowledge item not found: {}", id)))?;
        if let Some(title) = command.title {
            if title.trim().is_empty() {
                return Err(DomainError::ValidationError("Title cannot be empty".into()));
            }
            item.title = title;
        }
        if let Some(content) = command.content {
            if content.trim().is_empty() {
                return Err(DomainError::ValidationError("Content cannot be empty".into()));
            }
            item.embedding = Some(self.embed(&content).await?);
            item.content = content;
        }
        if let Some(metadata) = command.metadata {
            item.metadata = Some(metadata);
        }
        if let Some(is_active) = command.is_active {
            item.is_active = is_active;
        }
        item.updated_at = Utc::now();
        self.knowledge_repository.update(&item).await?;
        Ok(Self::to_response(&item))
    }

    async fn delete_knowledge_item(&self, id: Uuid) -> Result<(), DomainError> {
        info!(item_id = %id, "Deleting knowledge item");
        let deleted = self.knowledge_repository.delete_by_id(id).await?;
        if !deleted {
            warn!(item_id = %id, "Knowledge item not found for deletion");
            return Err(DomainError::NotFound(format!("Knowledge item not found: {}", id)));
        }
        Ok(())
    }
}
