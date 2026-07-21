use std::sync::Arc;
use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;
use sdlc_domain::dto::KnowledgeItem::KnowledgeItem;
use sdlc_domain::dto::KnowledgeItemResponse::KnowledgeItemResponse;
use sdlc_domain::dto::KnowledgeSearchResult::KnowledgeSearchResult;
use sdlc_domain::dto::SemanticSearchCommand::SemanticSearchCommand;
use sdlc_domain::port::input::SemanticSearchPort::SemanticSearchPort;
use sdlc_domain::port::output::EmbeddingProviderPort::EmbeddingProviderPort;
use sdlc_domain::port::output::KnowledgeRepositoryPort::KnowledgeRepositoryPort;
use sdlc_domain::r#enum::DomainError::DomainError;
use sdlc_domain::service::KnowledgeSearchService::KnowledgeSearchService;

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
    async fn update(&self, _item: &KnowledgeItem) -> Result<(), DomainError> {
        Ok(())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<KnowledgeItem>, DomainError> {
        Ok(self.items.lock().unwrap().iter().find(|i| i.id == id).cloned())
    }
    async fn find_all_by_key_and_tenant(&self, _tenant_id: Uuid, _key: &str) -> Result<Vec<KnowledgeItem>, DomainError> {
        Ok(Vec::new())
    }
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<KnowledgeItem>, DomainError> {
        Ok(self.items.lock().unwrap().iter().filter(|i| i.tenant_id == tenant_id).cloned().collect())
    }
    async fn delete_by_id(&self, _id: Uuid) -> Result<bool, DomainError> {
        Ok(false)
    }

    /// In-memory stand-in for the pgvector-backed cosine ranking the real
    /// repository performs in the database; keeps this test suite meaningful
    /// (ordering, filtering, limiting) without needing a live Postgres instance.
    async fn find_nearest_by_tenant(
        &self,
        tenant_id: Uuid,
        query_embedding: &[f32],
        source_type: Option<&str>,
        limit: u64,
    ) -> Result<Vec<(KnowledgeItem, f32)>, DomainError> {
        fn cosine_similarity(a: &[f32], b: &[f32]) -> Option<f32> {
            if a.is_empty() || a.len() != b.len() {
                return None;
            }
            let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
            let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
            let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
            if norm_a == 0.0 || norm_b == 0.0 { None } else { Some(dot / (norm_a * norm_b)) }
        }

        let mut scored: Vec<(KnowledgeItem, f32)> = self.items.lock().unwrap().iter()
            .filter(|i| i.tenant_id == tenant_id && i.is_active)
            .filter(|i| source_type.is_none_or(|st| i.source_type == st))
            .filter_map(|i| {
                let embedding = i.embedding.as_deref()?;
                let score = cosine_similarity(query_embedding, embedding)?;
                Some((i.clone(), score))
            })
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(limit as usize);
        Ok(scored)
    }
}

/// Looks up a pre-registered vector for known strings, so tests can control similarity precisely.
#[derive(Default)]
struct LookupEmbeddingProvider {
    vectors: std::collections::HashMap<String, Vec<f32>>,
}

#[async_trait]
impl EmbeddingProviderPort for LookupEmbeddingProvider {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, DomainError> {
        self.vectors.get(text).cloned()
            .ok_or_else(|| DomainError::InternalError(format!("No stub vector for '{}'", text)))
    }
}

fn item_with_embedding(tenant_id: Uuid, key: &str, source_type: &str, embedding: Vec<f32>) -> KnowledgeItem {
    use sdlc_domain::dto::KnowledgeItem::NewKnowledgeItem;
    let mut item = KnowledgeItem::new(NewKnowledgeItem {
        tenant_id,
        source_type: source_type.into(),
        key: key.into(),
        version: 1,
        title: key.into(),
        content: key.into(),
        metadata: None,
        embedding: Some(embedding),
    });
    item.is_active = true;
    item
}

#[tokio::test]
async fn search_ranks_by_similarity_descending() {
    let tenant_id = Uuid::new_v4();
    let repo = Arc::new(MockKnowledgeRepository::default());
    repo.items.lock().unwrap().push(item_with_embedding(tenant_id, "close", "adr", vec![1.0, 0.0]));
    repo.items.lock().unwrap().push(item_with_embedding(tenant_id, "far", "adr", vec![0.0, 1.0]));

    let mut vectors = std::collections::HashMap::new();
    vectors.insert("find something".to_string(), vec![1.0, 0.0]);
    let provider = Arc::new(LookupEmbeddingProvider { vectors });

    let service = KnowledgeSearchService::new(repo, provider);
    let results = service.search(tenant_id, SemanticSearchCommand {
        query: "find something".into(),
        source_type: None,
        limit: None,
    }).await.unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].item.key, "close");
    assert!(results[0].score > results[1].score);
}

#[tokio::test]
async fn search_filters_by_source_type() {
    let tenant_id = Uuid::new_v4();
    let repo = Arc::new(MockKnowledgeRepository::default());
    repo.items.lock().unwrap().push(item_with_embedding(tenant_id, "a", "adr", vec![1.0, 0.0]));
    repo.items.lock().unwrap().push(item_with_embedding(tenant_id, "b", "pull_request", vec![1.0, 0.0]));

    let mut vectors = std::collections::HashMap::new();
    vectors.insert("q".to_string(), vec![1.0, 0.0]);
    let provider = Arc::new(LookupEmbeddingProvider { vectors });

    let service = KnowledgeSearchService::new(repo, provider);
    let results = service.search(tenant_id, SemanticSearchCommand {
        query: "q".into(),
        source_type: Some("adr".into()),
        limit: None,
    }).await.unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].item.key, "a");
}

#[tokio::test]
async fn search_fails_when_query_is_empty() {
    let repo = Arc::new(MockKnowledgeRepository::default());
    let provider = Arc::new(LookupEmbeddingProvider::default());
    let service = KnowledgeSearchService::new(repo, provider);
    let result = service.search(Uuid::new_v4(), SemanticSearchCommand {
        query: "".into(),
        source_type: None,
        limit: None,
    }).await;
    assert!(matches!(result, Err(DomainError::ValidationError(_))));
}
