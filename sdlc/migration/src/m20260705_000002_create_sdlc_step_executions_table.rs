use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SdlcStepExecutions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SdlcStepExecutions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(SdlcStepExecutions::RunId).uuid().not_null())
                    .col(ColumnDef::new(SdlcStepExecutions::Step).string_len(50).not_null())
                    .col(ColumnDef::new(SdlcStepExecutions::Attempt).integer().not_null())
                    .col(ColumnDef::new(SdlcStepExecutions::Status).string_len(50).not_null())
                    .col(ColumnDef::new(SdlcStepExecutions::Output).text())
                    .col(ColumnDef::new(SdlcStepExecutions::Error).text())
                    .col(ColumnDef::new(SdlcStepExecutions::StartedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(SdlcStepExecutions::CompletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(SdlcStepExecutions::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(SdlcStepExecutions::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .index(
                        Index::create()
                            .unique()
                            .col(SdlcStepExecutions::RunId)
                            .col(SdlcStepExecutions::Step)
                            .col(SdlcStepExecutions::Attempt),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SdlcStepExecutions::Table, SdlcStepExecutions::RunId)
                            .to(SdlcRuns::Table, SdlcRuns::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(SdlcStepExecutions::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum SdlcStepExecutions {
    Table,
    Id,
    RunId,
    Step,
    Attempt,
    Status,
    Output,
    Error,
    StartedAt,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum SdlcRuns {
    Table,
    Id,
}
