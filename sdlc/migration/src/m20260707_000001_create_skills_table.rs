use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Skills::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Skills::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Skills::TenantId).uuid())
                    .col(ColumnDef::new(Skills::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Skills::Description).text().not_null())
                    .col(ColumnDef::new(Skills::Content).text().not_null())
                    .col(ColumnDef::new(Skills::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Skills::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Skills::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Skills::CreatedBy).uuid())
                    .col(ColumnDef::new(Skills::UpdatedBy).uuid())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Skills::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Skills {
    Table,
    Id,
    TenantId,
    Name,
    Description,
    Content,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
