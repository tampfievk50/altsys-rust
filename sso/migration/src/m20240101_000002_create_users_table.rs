use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Users::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Users::Username).string_len(100).not_null())
                    .col(ColumnDef::new(Users::Email).string_len(255).not_null())
                    .col(ColumnDef::new(Users::PasswordHash).text().not_null())
                    .col(ColumnDef::new(Users::FullName).string_len(255))
                    .col(ColumnDef::new(Users::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Users::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Users::CreatedBy).uuid())
                    .col(ColumnDef::new(Users::UpdatedBy).uuid())
                    .index(
                        Index::create()
                            .unique()
                            .col(Users::TenantId)
                            .col(Users::Username),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .col(Users::TenantId)
                            .col(Users::Email),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Users::Table, Users::TenantId)
                            .to(Alias::new("tenants"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    TenantId,
    Username,
    Email,
    PasswordHash,
    FullName,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
