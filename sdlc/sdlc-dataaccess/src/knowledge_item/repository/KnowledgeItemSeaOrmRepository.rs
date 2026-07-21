use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, FromQueryResult, ModelTrait, Statement};
use uuid::Uuid;

use crate::knowledge_item::entity::KnowledgeItemEntity;
use crate::knowledge_item::entity::KnowledgeItemEntity::Model;
use crate::knowledge_item::mapper::KnowledgeItemDataMapper::KnowledgeItemDataMapper;

/// One `knowledge_items` row plus its embedding cast to text (`"[0.1,-0.2,...]"`,
/// pgvector's textual format), read via raw SQL since SeaORM's entity system has
/// no native `vector` column type.
#[derive(Debug, FromQueryResult)]
pub struct KnowledgeItemRow {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub source_type: String,
    pub key: String,
    pub version: i32,
    pub title: String,
    pub content: String,
    pub metadata: Option<String>,
    pub embedding: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

/// A row ranked by pgvector's `<=>` cosine-distance operator; `distance` is `0.0`
/// for an identical vector and `2.0` for the opposite direction.
#[derive(Debug, FromQueryResult)]
pub struct KnowledgeItemNearestRow {
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
    pub distance: f64,
}

const ROW_COLUMNS: &str = "id, tenant_id, source_type, key, version, title, content, metadata, \
    embedding::text AS embedding, is_active, created_at, updated_at, created_by, updated_by";

pub struct KnowledgeItemSeaOrmRepository {
    db: DatabaseConnection,
}

impl KnowledgeItemSeaOrmRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, model: KnowledgeItemEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.insert(&self.db).await
    }

    pub async fn update(&self, model: KnowledgeItemEntity::ActiveModel) -> Result<Model, sea_orm::DbErr> {
        model.update(&self.db).await
    }

    /// Writes the embedding via a separate statement (`::vector` cast) since it
    /// isn't part of the entity's ActiveModel. Not run in the same transaction as
    /// `insert`/`update`, matching this codebase's existing baseline (see
    /// `docs/architecture.md` §9, "no explicit multi-statement transactions").
    pub async fn set_embedding(&self, id: Uuid, embedding: &[f32]) -> Result<(), sea_orm::DbErr> {
        let stmt = Statement::from_sql_and_values(
            self.db.get_database_backend(),
            "UPDATE knowledge_items SET embedding = $1::vector WHERE id = $2",
            [KnowledgeItemDataMapper::to_vector_literal(embedding).into(), id.into()],
        );
        self.db.execute(stmt).await.map(|_| ())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<KnowledgeItemRow>, sea_orm::DbErr> {
        let sql = format!("SELECT {} FROM knowledge_items WHERE id = $1", ROW_COLUMNS);
        let stmt = Statement::from_sql_and_values(self.db.get_database_backend(), &sql, [id.into()]);
        KnowledgeItemRow::find_by_statement(stmt).one(&self.db).await
    }

    pub async fn find_all_by_key_and_tenant(&self, tenant_id: Uuid, key: &str) -> Result<Vec<KnowledgeItemRow>, sea_orm::DbErr> {
        let sql = format!("SELECT {} FROM knowledge_items WHERE tenant_id = $1 AND key = $2", ROW_COLUMNS);
        let stmt = Statement::from_sql_and_values(self.db.get_database_backend(), &sql, [tenant_id.into(), key.into()]);
        KnowledgeItemRow::find_by_statement(stmt).all(&self.db).await
    }

    pub async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<KnowledgeItemRow>, sea_orm::DbErr> {
        let sql = format!("SELECT {} FROM knowledge_items WHERE tenant_id = $1", ROW_COLUMNS);
        let stmt = Statement::from_sql_and_values(self.db.get_database_backend(), &sql, [tenant_id.into()]);
        KnowledgeItemRow::find_by_statement(stmt).all(&self.db).await
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<bool, sea_orm::DbErr> {
        match KnowledgeItemEntity::Entity::find_by_id(id).one(&self.db).await? {
            Some(model) => { model.delete(&self.db).await?; Ok(true) }
            None => Ok(false),
        }
    }

    /// Ranks the latest, active version of every key by cosine distance to
    /// `embedding` using pgvector's `<=>` operator (accelerated by the HNSW index
    /// created in the migration), optionally restricted to one `source_type`.
    pub async fn find_nearest(
        &self,
        tenant_id: Uuid,
        embedding: &[f32],
        source_type: Option<&str>,
        limit: u64,
    ) -> Result<Vec<KnowledgeItemNearestRow>, sea_orm::DbErr> {
        let vector_literal = KnowledgeItemDataMapper::to_vector_literal(embedding);
        let mut values: Vec<sea_orm::Value> = vec![tenant_id.into(), vector_literal.into(), (limit as i64).into()];

        let mut sql = String::from(
            "WITH latest AS ( \
                SELECT DISTINCT ON (key) * FROM knowledge_items \
                WHERE tenant_id = $1 AND is_active = true AND embedding IS NOT NULL \
                ORDER BY key, version DESC \
            ) \
            SELECT id, tenant_id, source_type, key, version, title, content, metadata, is_active, \
                   created_at, updated_at, created_by, updated_by, \
                   (embedding <=> $2::vector) AS distance \
            FROM latest",
        );
        if let Some(source_type) = source_type {
            sql.push_str(" WHERE source_type = $4");
            values.push(source_type.to_string().into());
        }
        sql.push_str(" ORDER BY embedding <=> $2::vector LIMIT $3");

        let stmt = Statement::from_sql_and_values(self.db.get_database_backend(), &sql, values);
        KnowledgeItemNearestRow::find_by_statement(stmt).all(&self.db).await
    }
}
