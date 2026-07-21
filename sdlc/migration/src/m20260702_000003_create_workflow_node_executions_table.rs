use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkflowNodeExecutions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WorkflowNodeExecutions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WorkflowNodeExecutions::WorkflowExecutionId).uuid().not_null())
                    .col(ColumnDef::new(WorkflowNodeExecutions::NodeId).string_len(255).not_null())
                    .col(ColumnDef::new(WorkflowNodeExecutions::Status).string_len(50).not_null())
                    .col(ColumnDef::new(WorkflowNodeExecutions::Attempt).integer().not_null())
                    .col(ColumnDef::new(WorkflowNodeExecutions::Input).text())
                    .col(ColumnDef::new(WorkflowNodeExecutions::Output).text())
                    .col(ColumnDef::new(WorkflowNodeExecutions::Error).text())
                    .col(ColumnDef::new(WorkflowNodeExecutions::StartedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(WorkflowNodeExecutions::CompletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(WorkflowNodeExecutions::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(WorkflowNodeExecutions::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .index(
                        Index::create()
                            .unique()
                            .col(WorkflowNodeExecutions::WorkflowExecutionId)
                            .col(WorkflowNodeExecutions::NodeId)
                            .col(WorkflowNodeExecutions::Attempt),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WorkflowNodeExecutions::Table, WorkflowNodeExecutions::WorkflowExecutionId)
                            .to(WorkflowExecutions::Table, WorkflowExecutions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(WorkflowNodeExecutions::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum WorkflowNodeExecutions {
    Table,
    Id,
    WorkflowExecutionId,
    NodeId,
    Status,
    Attempt,
    Input,
    Output,
    Error,
    StartedAt,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum WorkflowExecutions {
    Table,
    Id,
}
