use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Roles::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Roles::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Roles::Name).string_len(100).not_null())
                    .col(ColumnDef::new(Roles::Description).text())
                    .col(ColumnDef::new(Roles::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Roles::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Roles::CreatedBy).uuid())
                    .col(ColumnDef::new(Roles::UpdatedBy).uuid())
                    .index(
                        Index::create()
                            .unique()
                            .col(Roles::TenantId)
                            .col(Roles::Name),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Roles::Table, Roles::TenantId)
                            .to(Alias::new("tenants"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Roles::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    TenantId,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
