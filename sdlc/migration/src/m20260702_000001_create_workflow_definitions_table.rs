use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkflowDefinitions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WorkflowDefinitions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WorkflowDefinitions::TenantId).uuid().not_null())
                    .col(ColumnDef::new(WorkflowDefinitions::Key).string_len(255).not_null())
                    .col(ColumnDef::new(WorkflowDefinitions::Version).integer().not_null())
                    .col(ColumnDef::new(WorkflowDefinitions::Name).string_len(255).not_null())
                    .col(ColumnDef::new(WorkflowDefinitions::Description).text())
                    .col(ColumnDef::new(WorkflowDefinitions::Definition).text().not_null())
                    .col(ColumnDef::new(WorkflowDefinitions::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(WorkflowDefinitions::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(WorkflowDefinitions::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(WorkflowDefinitions::CreatedBy).uuid())
                    .col(ColumnDef::new(WorkflowDefinitions::UpdatedBy).uuid())
                    .index(
                        Index::create()
                            .unique()
                            .col(WorkflowDefinitions::TenantId)
                            .col(WorkflowDefinitions::Key)
                            .col(WorkflowDefinitions::Version),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(WorkflowDefinitions::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum WorkflowDefinitions {
    Table,
    Id,
    TenantId,
    Key,
    Version,
    Name,
    Description,
    Definition,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
