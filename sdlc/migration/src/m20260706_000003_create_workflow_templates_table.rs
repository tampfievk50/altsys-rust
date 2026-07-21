use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkflowTemplates::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WorkflowTemplates::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(WorkflowTemplates::TenantId).uuid().not_null())
                    .col(ColumnDef::new(WorkflowTemplates::Key).string_len(255).not_null())
                    .col(ColumnDef::new(WorkflowTemplates::Version).integer().not_null())
                    .col(ColumnDef::new(WorkflowTemplates::Name).string_len(255).not_null())
                    .col(ColumnDef::new(WorkflowTemplates::Description).text())
                    .col(ColumnDef::new(WorkflowTemplates::DefinitionTemplate).text().not_null())
                    .col(ColumnDef::new(WorkflowTemplates::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(WorkflowTemplates::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(WorkflowTemplates::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(WorkflowTemplates::CreatedBy).uuid())
                    .col(ColumnDef::new(WorkflowTemplates::UpdatedBy).uuid())
                    .index(
                        Index::create()
                            .unique()
                            .col(WorkflowTemplates::TenantId)
                            .col(WorkflowTemplates::Key)
                            .col(WorkflowTemplates::Version),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(WorkflowTemplates::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum WorkflowTemplates {
    Table,
    Id,
    TenantId,
    Key,
    Version,
    Name,
    Description,
    DefinitionTemplate,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
