use sea_orm_migration::prelude::*;

use crate::m20260723_000001_create_schedulers_table::Schedulers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(JobExecutions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(JobExecutions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(JobExecutions::SchedulerId).uuid().not_null())
                    .col(ColumnDef::new(JobExecutions::TriggerType).string_len(20).not_null())
                    .col(ColumnDef::new(JobExecutions::Status).string_len(20).not_null())
                    .col(ColumnDef::new(JobExecutions::StartedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(JobExecutions::FinishedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(JobExecutions::StatusCode).integer())
                    .col(ColumnDef::new(JobExecutions::ResponseBody).text())
                    .col(ColumnDef::new(JobExecutions::ErrorMessage).text())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_job_executions_scheduler_id")
                            .from(JobExecutions::Table, JobExecutions::SchedulerId)
                            .to(Schedulers::Table, Schedulers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_job_executions_scheduler_id")
                    .table(JobExecutions::Table)
                    .col(JobExecutions::SchedulerId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(JobExecutions::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum JobExecutions {
    Table,
    Id,
    SchedulerId,
    TriggerType,
    Status,
    StartedAt,
    FinishedAt,
    StatusCode,
    ResponseBody,
    ErrorMessage,
}
