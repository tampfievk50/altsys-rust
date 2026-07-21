use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tools::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tools::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Tools::TenantId).uuid())
                    .col(ColumnDef::new(Tools::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Tools::ToolType).string_len(50).not_null())
                    .col(ColumnDef::new(Tools::Description).text())
                    .col(ColumnDef::new(Tools::Config).text())
                    .col(ColumnDef::new(Tools::IsEnabled).boolean().not_null().default(true))
                    .col(ColumnDef::new(Tools::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Tools::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Tools::CreatedBy).uuid())
                    .col(ColumnDef::new(Tools::UpdatedBy).uuid())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Tools::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Tools {
    Table,
    Id,
    TenantId,
    Name,
    ToolType,
    Description,
    Config,
    IsEnabled,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
