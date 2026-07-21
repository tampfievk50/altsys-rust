use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AgentExecutions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AgentExecutions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AgentExecutions::TenantId).uuid().not_null())
                    .col(ColumnDef::new(AgentExecutions::AgentId).uuid().not_null())
                    .col(ColumnDef::new(AgentExecutions::Input).text().not_null())
                    .col(ColumnDef::new(AgentExecutions::Output).text())
                    .col(ColumnDef::new(AgentExecutions::Status).string_len(50).not_null())
                    .col(ColumnDef::new(AgentExecutions::Error).text())
                    .col(ColumnDef::new(AgentExecutions::StartedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(AgentExecutions::CompletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(AgentExecutions::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(AgentExecutions::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(AgentExecutions::Table, AgentExecutions::AgentId)
                            .to(Agents::Table, Agents::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AgentExecutions::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum AgentExecutions {
    Table,
    Id,
    TenantId,
    AgentId,
    Input,
    Output,
    Status,
    Error,
    StartedAt,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Agents {
    Table,
    Id,
}
