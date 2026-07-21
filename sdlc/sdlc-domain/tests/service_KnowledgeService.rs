use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use tracing::{info, warn};
use uuid::Uuid;
use sdlc_domain::dto::CreateKnowledgeItemCommand::CreateKnowledgeItemCommand;
use sdlc_domain::dto::KnowledgeItem::{KnowledgeItem, NewKnowledgeItem};
use sdlc_domain::dto::KnowledgeItemResponse::KnowledgeItemResponse;
use sdlc_domain::dto::UpdateKnowledgeItemCommand::UpdateKnowledgeItemCommand;
use sdlc_domain::port::input::KnowledgePort::KnowledgePort;
use sdlc_domain::port::output::EmbeddingProviderPort::EmbeddingProviderPort;
use sdlc_domain::port::output::KnowledgeRepositoryPort::KnowledgeRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::KnowledgeService::KnowledgeService;

use std::sync::Mutex;

#[derive(Default)]
struct MockKnowledgeRepository {
    items: Mutex<Vec<KnowledgeItem>>,
}

#[async_trait]
impl KnowledgeRepositoryPort for MockKnowledgeRepository {
    async fn save(&self, item: &KnowledgeItem) -> Result<(), DomainError> {
        self.items.lock().unwrap().push(item.clone());
        Ok(())
    }

    async fn update(&self, item: &KnowledgeItem) -> Result<(), DomainError> {
        let mut items = self.items.lock().unwrap();
        if let Some(existing) = items.iter_mut().find(|i| i.id == item.id) {
            *existing = item.clone();
        }
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<KnowledgeItem>, DomainError> {
        Ok(self.items.lock().unwrap().iter().find(|i| i.id == id).cloned())
    }

    async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<KnowledgeItem>, DomainError> {
        Ok(self.items.lock().unwrap().iter().filter(|i| i.tenant_id == tenant_id && i.key == key).cloned().collect())
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<KnowledgeItem>, DomainError> {
        Ok(self.items.lock().unwrap().iter().filter(|i| i.tenant_id == tenant_id).cloned().collect())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, DomainError> {
        let mut items = self.items.lock().unwrap();
        let len_before = items.len();
        items.retain(|i| i.id != id);
        Ok(items.len() != len_before)
    }

    async fn find_nearest_by_tenant(
        &self,
        _tenant_id: Uuid,
        _query_embedding: &[f32],
        _source_type: Option<&str>,
        _limit: u64,
    ) -> Result<Vec<(KnowledgeItem, f32)>, DomainError> {
        Ok(Vec::new())
    }
}

struct StubEmbeddingProvider;

#[async_trait]
impl EmbeddingProviderPort for StubEmbeddingProvider {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, DomainError> {
        Ok(vec![text.len() as f32, 1.0])
    }
}

fn service() -> KnowledgeService {
    KnowledgeService::new(Arc::new(MockKnowledgeRepository::default()), Arc::new(StubEmbeddingProvider))
}

fn sample_command(tenant_id: Uuid) -> CreateKnowledgeItemCommand {
    CreateKnowledgeItemCommand {
        tenant_id,
        source_type: "adr".into(),
        key: "adr-0007".into(),
        title: "Use hexagonal architecture".into(),
        content: "We will structure services as ports and adapters.".into(),
        metadata: None,
    }
}

#[tokio::test]
async fn create_knowledge_item_starts_at_version_one_and_embeds_content() {
    let service = service();
    let response = service.create_knowledge_item(sample_command(Uuid::new_v4())).await.unwrap();
    assert_eq!(response.version, 1);
    assert!(response.embedding.is_some());
}

#[tokio::test]
async fn create_knowledge_item_increments_version_for_the_same_key() {
    let service = service();
    let tenant_id = Uuid::new_v4();
    service.create_knowledge_item(sample_command(tenant_id)).await.unwrap();
    let second = service.create_knowledge_item(sample_command(tenant_id)).await.unwrap();
    assert_eq!(second.version, 2);
}

#[tokio::test]
async fn create_knowledge_item_fails_when_content_is_empty() {
    let service = service();
    let mut command = sample_command(Uuid::new_v4());
    command.content = "".into();
    let result = service.create_knowledge_item(command).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}

#[tokio::test]
async fn find_latest_knowledge_item_by_key_returns_highest_version() {
    let service = service();
    let tenant_id = Uuid::new_v4();
    service.create_knowledge_item(sample_command(tenant_id)).await.unwrap();
    service.create_knowledge_item(sample_command(tenant_id)).await.unwrap();
    let latest = service.find_latest_knowledge_item_by_key(tenant_id, "adr-0007").await.unwrap();
    assert_eq!(latest.version, 2);
}

#[tokio::test]
async fn find_knowledge_items_by_tenant_returns_latest_per_key_only() {
    let service = service();
    let tenant_id = Uuid::new_v4();
    service.create_knowledge_item(sample_command(tenant_id)).await.unwrap();
    service.create_knowledge_item(sample_command(tenant_id)).await.unwrap();
    let mut other_key_command = sample_command(tenant_id);
    other_key_command.key = "adr-0008".into();
    service.create_knowledge_item(other_key_command).await.unwrap();

    let results = service.find_knowledge_items_by_tenant(tenant_id).await.unwrap();
    assert_eq!(results.len(), 2);
    let adr7 = results.iter().find(|i| i.key == "adr-0007").unwrap();
    assert_eq!(adr7.version, 2);
}

#[tokio::test]
async fn update_knowledge_item_reembeds_when_content_changes() {
    let service = service();
    let created = service.create_knowledge_item(sample_command(Uuid::new_v4())).await.unwrap();
    let updated = service.update_knowledge_item(created.id, UpdateKnowledgeItemCommand {
        title: None,
        content: Some("A much longer replacement piece of content.".into()),
        metadata: None,
        is_active: None,
    }).await.unwrap();
    assert_ne!(updated.embedding, created.embedding);
    assert_eq!(updated.version, created.version);
}

#[tokio::test]
async fn delete_knowledge_item_fails_when_not_found() {
    let service = service();
    let result = service.delete_knowledge_item(Uuid::new_v4()).await;
    assert!(matches!(result, Err(DomainError::NotFound(_))));
}
