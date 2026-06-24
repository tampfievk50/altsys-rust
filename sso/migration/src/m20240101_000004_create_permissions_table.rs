use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permissions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Permissions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Permissions::Name).string_len(100).not_null().unique_key())
                    .col(ColumnDef::new(Permissions::Action).string_len(100).not_null())
                    .col(ColumnDef::new(Permissions::Resource).string_len(100).not_null())
                    .col(ColumnDef::new(Permissions::Description).text())
                    .col(ColumnDef::new(Permissions::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Permissions::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Permissions::CreatedBy).uuid())
                    .col(ColumnDef::new(Permissions::UpdatedBy).uuid())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Permissions::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Permissions {
    Table,
    Id,
    Name,
    Action,
    Resource,
    Description,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
