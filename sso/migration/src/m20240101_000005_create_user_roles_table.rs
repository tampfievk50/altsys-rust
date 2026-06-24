use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRoles::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserRoles::RoleId).uuid().not_null())
                    .col(ColumnDef::new(UserRoles::TenantId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .col(UserRoles::UserId)
                            .col(UserRoles::RoleId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRoles::Table, UserRoles::UserId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRoles::Table, UserRoles::RoleId)
                            .to(Alias::new("roles"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRoles::Table, UserRoles::TenantId)
                            .to(Alias::new("tenants"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(UserRoles::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum UserRoles {
    Table,
    UserId,
    RoleId,
    TenantId,
}
