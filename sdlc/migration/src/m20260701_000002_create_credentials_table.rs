use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Credentials::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Credentials::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Credentials::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Credentials::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Credentials::Provider).string_len(50).not_null())
                    .col(ColumnDef::new(Credentials::EncryptedSecret).text().not_null())
                    .col(ColumnDef::new(Credentials::SecretHint).string_len(20))
                    .col(ColumnDef::new(Credentials::Metadata).text())
                    .col(ColumnDef::new(Credentials::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Credentials::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Credentials::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Credentials::CreatedBy).uuid())
                    .col(ColumnDef::new(Credentials::UpdatedBy).uuid())
                    .index(
                        Index::create()
                            .unique()
                            .col(Credentials::TenantId)
                            .col(Credentials::Name),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Credentials::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Credentials {
    Table,
    Id,
    TenantId,
    Name,
    Provider,
    EncryptedSecret,
    SecretHint,
    Metadata,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
