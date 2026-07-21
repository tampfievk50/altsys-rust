use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Plugins::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Plugins::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Plugins::TenantId).uuid())
                    .col(ColumnDef::new(Plugins::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Plugins::WebhookUrl).string_len(500).not_null())
                    .col(ColumnDef::new(Plugins::Secret).string_len(255))
                    .col(ColumnDef::new(Plugins::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Plugins::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Plugins::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Plugins::CreatedBy).uuid())
                    .col(ColumnDef::new(Plugins::UpdatedBy).uuid())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Plugins::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Plugins {
    Table,
    Id,
    TenantId,
    Name,
    WebhookUrl,
    Secret,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
