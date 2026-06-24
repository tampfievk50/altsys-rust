use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Downloads::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Downloads::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Downloads::VideoUrl).text().not_null())
                    .col(ColumnDef::new(Downloads::VideoId).text())
                    .col(ColumnDef::new(Downloads::Title).text())
                    .col(ColumnDef::new(Downloads::Status).string_len(50).not_null().default("PENDING"))
                    .col(ColumnDef::new(Downloads::FilePath).text())
                    .col(ColumnDef::new(Downloads::FileSizeBytes).big_integer())
                    .col(ColumnDef::new(Downloads::ErrorMessage).text())
                    .col(
                        ColumnDef::new(Downloads::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Downloads::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Downloads::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Downloads {
    Table,
    Id,
    VideoUrl,
    VideoId,
    Title,
    Status,
    FilePath,
    FileSizeBytes,
    ErrorMessage,
    CreatedAt,
    UpdatedAt,
}
