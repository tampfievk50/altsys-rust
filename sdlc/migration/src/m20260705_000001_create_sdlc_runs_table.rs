use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SdlcRuns::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SdlcRuns::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(SdlcRuns::TenantId).uuid().not_null())
                    .col(ColumnDef::new(SdlcRuns::ProjectId).uuid().not_null())
                    .col(ColumnDef::new(SdlcRuns::TicketKey).string_len(100).not_null())
                    .col(ColumnDef::new(SdlcRuns::Status).string_len(50).not_null())
                    .col(ColumnDef::new(SdlcRuns::CurrentStep).string_len(50))
                    .col(ColumnDef::new(SdlcRuns::BranchName).string_len(255))
                    .col(ColumnDef::new(SdlcRuns::PullRequestUrl).string_len(500))
                    .col(ColumnDef::new(SdlcRuns::Context).text().not_null())
                    .col(ColumnDef::new(SdlcRuns::Error).text())
                    .col(ColumnDef::new(SdlcRuns::StartedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(SdlcRuns::CompletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(SdlcRuns::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(SdlcRuns::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(SdlcRuns::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum SdlcRuns {
    Table,
    Id,
    TenantId,
    ProjectId,
    TicketKey,
    Status,
    CurrentStep,
    BranchName,
    PullRequestUrl,
    Context,
    Error,
    StartedAt,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
}
