use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(KnowledgeItems::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(KnowledgeItems::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(KnowledgeItems::TenantId).uuid().not_null())
                    .col(ColumnDef::new(KnowledgeItems::SourceType).string_len(50).not_null())
                    .col(ColumnDef::new(KnowledgeItems::Key).string_len(255).not_null())
                    .col(ColumnDef::new(KnowledgeItems::Version).integer().not_null())
                    .col(ColumnDef::new(KnowledgeItems::Title).string_len(500).not_null())
                    .col(ColumnDef::new(KnowledgeItems::Content).text().not_null())
                    .col(ColumnDef::new(KnowledgeItems::Metadata).text())
                    // Stored as a JSON-encoded float array for now (portable, no DB extension
                    // required). Swap for a native `vector` column + pgvector once the extension
                    // is installed; see EmbeddingProviderPort / KnowledgeSearchService.
                    .col(ColumnDef::new(KnowledgeItems::Embedding).text())
                    .col(ColumnDef::new(KnowledgeItems::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(KnowledgeItems::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(KnowledgeItems::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(KnowledgeItems::CreatedBy).uuid())
                    .col(ColumnDef::new(KnowledgeItems::UpdatedBy).uuid())
                    .index(
                        Index::create()
                            .unique()
                            .col(KnowledgeItems::TenantId)
                            .col(KnowledgeItems::Key)
                            .col(KnowledgeItems::Version),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(KnowledgeItems::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum KnowledgeItems {
    Table,
    Id,
    TenantId,
    SourceType,
    Key,
    Version,
    Title,
    Content,
    Metadata,
    Embedding,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
