use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Schedulers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Schedulers::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Schedulers::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Schedulers::Description).text())
                    .col(ColumnDef::new(Schedulers::CronExpression).string_len(100).not_null())
                    .col(ColumnDef::new(Schedulers::WebhookUrl).text().not_null())
                    .col(ColumnDef::new(Schedulers::HttpMethod).string_len(10).not_null())
                    .col(ColumnDef::new(Schedulers::Headers).json_binary())
                    .col(ColumnDef::new(Schedulers::Body).json_binary())
                    .col(ColumnDef::new(Schedulers::TimeoutSeconds).integer().not_null().default(30))
                    .col(ColumnDef::new(Schedulers::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Schedulers::NextRunAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Schedulers::LastRunAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Schedulers::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Schedulers::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Schedulers::CreatedBy).uuid())
                    .col(ColumnDef::new(Schedulers::UpdatedBy).uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_schedulers_due")
                    .table(Schedulers::Table)
                    .col(Schedulers::IsActive)
                    .col(Schedulers::NextRunAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Schedulers::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum Schedulers {
    Table,
    Id,
    Name,
    Description,
    CronExpression,
    WebhookUrl,
    HttpMethod,
    Headers,
    Body,
    TimeoutSeconds,
    IsActive,
    NextRunAt,
    LastRunAt,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
