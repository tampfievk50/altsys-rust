use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Models::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Models::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Models::TenantId).uuid())
                    .col(ColumnDef::new(Models::Provider).string_len(50).not_null())
                    .col(ColumnDef::new(Models::ModelName).string_len(255).not_null())
                    .col(ColumnDef::new(Models::Capability).string_len(50).not_null())
                    .col(ColumnDef::new(Models::CredentialId).uuid())
                    .col(ColumnDef::new(Models::EndpointUrl).string_len(500))
                    .col(ColumnDef::new(Models::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Models::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Models::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Models::CreatedBy).uuid())
                    .col(ColumnDef::new(Models::UpdatedBy).uuid())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Models::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Models {
    Table,
    Id,
    TenantId,
    Provider,
    ModelName,
    Capability,
    CredentialId,
    EndpointUrl,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
