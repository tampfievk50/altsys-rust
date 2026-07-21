use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Agents::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Agents::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Agents::TenantId).uuid())
                    .col(ColumnDef::new(Agents::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Agents::AgentType).string_len(50).not_null())
                    .col(ColumnDef::new(Agents::SystemPrompt).text().not_null())
                    .col(ColumnDef::new(Agents::Provider).string_len(50).not_null())
                    .col(ColumnDef::new(Agents::Model).string_len(100).not_null())
                    .col(ColumnDef::new(Agents::Temperature).float())
                    .col(ColumnDef::new(Agents::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Agents::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Agents::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Agents::CreatedBy).uuid())
                    .col(ColumnDef::new(Agents::UpdatedBy).uuid())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Agents::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Agents {
    Table,
    Id,
    TenantId,
    Name,
    AgentType,
    SystemPrompt,
    Provider,
    Model,
    Temperature,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
