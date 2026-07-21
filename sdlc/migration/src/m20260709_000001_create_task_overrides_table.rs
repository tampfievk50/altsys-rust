use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaskOverrides::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TaskOverrides::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(TaskOverrides::ProjectId).uuid().not_null())
                    .col(ColumnDef::new(TaskOverrides::TicketKey).string_len(50).not_null())
                    .col(ColumnDef::new(TaskOverrides::Summary).text().not_null())
                    .col(ColumnDef::new(TaskOverrides::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(TaskOverrides::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .index(
                        Index::create()
                            .unique()
                            .col(TaskOverrides::ProjectId)
                            .col(TaskOverrides::TicketKey),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TaskOverrides::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum TaskOverrides {
    Table,
    Id,
    ProjectId,
    TicketKey,
    Summary,
    CreatedAt,
    UpdatedAt,
}
