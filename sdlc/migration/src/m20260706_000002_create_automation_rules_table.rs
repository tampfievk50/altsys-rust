use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AutomationRules::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AutomationRules::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AutomationRules::TenantId).uuid().not_null())
                    .col(ColumnDef::new(AutomationRules::Name).string_len(255).not_null())
                    .col(ColumnDef::new(AutomationRules::EventType).string_len(100).not_null())
                    .col(ColumnDef::new(AutomationRules::MatchCriteria).text())
                    .col(ColumnDef::new(AutomationRules::Action).text().not_null())
                    .col(ColumnDef::new(AutomationRules::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(AutomationRules::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(AutomationRules::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(AutomationRules::CreatedBy).uuid())
                    .col(ColumnDef::new(AutomationRules::UpdatedBy).uuid())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(AutomationRules::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum AutomationRules {
    Table,
    Id,
    TenantId,
    Name,
    EventType,
    MatchCriteria,
    Action,
    IsActive,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
