use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct KnowledgeItemResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub source_type: String,
    pub key: String,
    pub version: i32,
    pub title: String,
    pub content: String,
    pub metadata: Option<String>,
    pub embedding: Option<Vec<f32>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}
