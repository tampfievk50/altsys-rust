use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tenants::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tenants::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Tenants::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Tenants::Slug).string_len(100).not_null().unique_key())
                    .col(ColumnDef::new(Tenants::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Tenants::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Tenants::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Tenants::CreatedBy).uuid())
                    .col(ColumnDef::new(Tenants::UpdatedBy).uuid())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Tenants::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Tenants {
    Table,
    Id,
    Name,
    Slug,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
