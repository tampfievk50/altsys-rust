use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct KnowledgeItem {
    pub id: Uuid,
    pub tenant_id: Uuid,
    /// e.g. `ticket`, `pull_request`, `adr`, `readme`, `wiki`, `implementation_plan`, `lessons_learned`.
    pub source_type: String,
    /// Stable logical identifier shared by every version of the same item (e.g. `PROJ-123`, `adr-0007`).
    pub key: String,
    pub version: i32,
    pub title: String,
    pub content: String,
    pub metadata: Option<String>,
    /// Produced by an `EmbeddingProviderPort`; stored as a native `vector` column in Postgres
    /// (pgvector) by the dataaccess layer.
    pub embedding: Option<Vec<f32>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

pub struct NewKnowledgeItem {
    pub tenant_id: Uuid,
    pub source_type: String,
    pub key: String,
    pub version: i32,
    pub title: String,
    pub content: String,
    pub metadata: Option<String>,
    pub embedding: Option<Vec<f32>>,
}

impl KnowledgeItem {
    /// Reduces a set of knowledge item rows (possibly several versions per key) down to
    /// the single highest-version row for each distinct key.
    pub fn latest_per_key(items: Vec<KnowledgeItem>) -> Vec<KnowledgeItem> {
        use std::collections::HashMap;
        let mut latest: HashMap<String, KnowledgeItem> = HashMap::new();
        for item in items {
            latest
                .entry(item.key.clone())
                .and_modify(|existing| {
                    if item.version > existing.version {
                        *existing = item.clone();
                    }
                })
                .or_insert(item);
        }
        latest.into_values().collect()
    }

    pub fn new(fields: NewKnowledgeItem) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: fields.tenant_id,
            source_type: fields.source_type,
            key: fields.key,
            version: fields.version,
            title: fields.title,
            content: fields.content,
            metadata: fields.metadata,
            embedding: fields.embedding,
            is_active: true,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}
