use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Projects::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Projects::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Projects::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Projects::Slug).string_len(100).not_null())
                    .col(ColumnDef::new(Projects::GithubToolId).uuid().not_null())
                    .col(ColumnDef::new(Projects::DefaultBranch).string_len(100).not_null().default("main"))
                    .col(ColumnDef::new(Projects::JiraToolId).uuid())
                    .col(ColumnDef::new(Projects::BuildCommand).text())
                    .col(ColumnDef::new(Projects::TestCommand).text())
                    .col(ColumnDef::new(Projects::CodingStandards).text())
                    .col(ColumnDef::new(Projects::WorkflowConfig).text())
                    .col(ColumnDef::new(Projects::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Projects::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Projects::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Projects::CreatedBy).uuid())
                    .col(ColumnDef::new(Projects::UpdatedBy).uuid())
                    .index(
                        Index::create()
                            .unique()
                            .col(Projects::TenantId)
                            .col(Projects::Slug),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Projects::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
    TenantId,
    Name,
    Slug,
    GithubToolId,
    DefaultBranch,
    JiraToolId,
    BuildCommand,
    TestCommand,
    CodingStandards,
    WorkflowConfig,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
