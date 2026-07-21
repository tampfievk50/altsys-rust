use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Prompts::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Prompts::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Prompts::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Prompts::Key).string_len(255).not_null())
                    .col(ColumnDef::new(Prompts::Version).integer().not_null())
                    .col(ColumnDef::new(Prompts::Content).text().not_null())
                    .col(ColumnDef::new(Prompts::Variables).text())
                    .col(ColumnDef::new(Prompts::Description).text())
                    .col(ColumnDef::new(Prompts::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Prompts::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Prompts::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Prompts::CreatedBy).uuid())
                    .col(ColumnDef::new(Prompts::UpdatedBy).uuid())
                    .index(
                        Index::create()
                            .unique()
                            .col(Prompts::TenantId)
                            .col(Prompts::Key)
                            .col(Prompts::Version),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Prompts::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Prompts {
    Table,
    Id,
    TenantId,
    Key,
    Version,
    Content,
    Variables,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
