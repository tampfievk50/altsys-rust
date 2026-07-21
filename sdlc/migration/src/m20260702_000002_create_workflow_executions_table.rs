use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkflowExecutions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WorkflowExecutions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WorkflowExecutions::TenantId).uuid().not_null())
                    .col(ColumnDef::new(WorkflowExecutions::WorkflowDefinitionId).uuid().not_null())
                    .col(ColumnDef::new(WorkflowExecutions::Status).string_len(50).not_null())
                    .col(ColumnDef::new(WorkflowExecutions::Context).text().not_null())
                    .col(ColumnDef::new(WorkflowExecutions::Error).text())
                    .col(ColumnDef::new(WorkflowExecutions::StartedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(WorkflowExecutions::CompletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(WorkflowExecutions::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(WorkflowExecutions::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(WorkflowExecutions::CreatedBy).uuid())
                    .col(ColumnDef::new(WorkflowExecutions::UpdatedBy).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .from(WorkflowExecutions::Table, WorkflowExecutions::WorkflowDefinitionId)
                            .to(WorkflowDefinitions::Table, WorkflowDefinitions::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(WorkflowExecutions::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum WorkflowExecutions {
    Table,
    Id,
    TenantId,
    WorkflowDefinitionId,
    Status,
    Context,
    Error,
    StartedAt,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}

#[derive(DeriveIden)]
enum WorkflowDefinitions {
    Table,
    Id,
}
