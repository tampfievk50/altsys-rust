use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// Embedding dimensionality of the default `LocalHashEmbeddingProvider`. A real
/// model-backed provider with a different dimension would need its own migration
/// (pgvector columns are fixed-width).
const EMBEDDING_DIMENSIONS: i32 = 256;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // sea-query has no `vector` column type, so the extension, column swap, and
        // index all go through raw SQL — the one carve-out from "no raw SQL" this
        // project's conventions call for when the query builder simply can't express
        // something (see docs/database-schema.md's own note on this pattern).
        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS vector").await?;

        manager
            .alter_table(
                Table::alter()
                    .table(KnowledgeItems::Table)
                    .drop_column(KnowledgeItems::Embedding)
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(&format!(
            "ALTER TABLE knowledge_items ADD COLUMN embedding vector({})",
            EMBEDDING_DIMENSIONS
        ))
        .await?;

        db.execute_unprepared(
            "CREATE INDEX knowledge_items_embedding_hnsw_idx \
             ON knowledge_items USING hnsw (embedding vector_cosine_ops)",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DROP INDEX IF EXISTS knowledge_items_embedding_hnsw_idx").await?;

        manager
            .alter_table(
                Table::alter()
                    .table(KnowledgeItems::Table)
                    .drop_column(KnowledgeItems::Embedding)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(KnowledgeItems::Table)
                    .add_column(ColumnDef::new(KnowledgeItems::Embedding).text())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum KnowledgeItems {
    Table,
    Embedding,
}
