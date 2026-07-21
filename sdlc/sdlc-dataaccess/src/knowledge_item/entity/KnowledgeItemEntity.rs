use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

/// Deliberately excludes the `embedding` column: it's a native pgvector `vector`
/// type that SeaORM's entity/ActiveModel system has no column type for. It is
/// managed separately via raw SQL (see `KnowledgeItemSeaOrmRepository`), which also
/// lets reads cast it to text and writes bind it with an explicit `::vector` cast.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "knowledge_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub source_type: String,
    pub key: String,
    pub version: i32,
    pub title: String,
    pub content: String,
    pub metadata: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
