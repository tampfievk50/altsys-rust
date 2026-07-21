use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RuleFirings::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RuleFirings::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(RuleFirings::EventId).uuid().not_null())
                    .col(ColumnDef::new(RuleFirings::RuleId).uuid().not_null())
                    .col(ColumnDef::new(RuleFirings::Matched).boolean().not_null())
                    .col(ColumnDef::new(RuleFirings::Status).string_len(50).not_null())
                    .col(ColumnDef::new(RuleFirings::ActionResult).text())
                    .col(ColumnDef::new(RuleFirings::Error).text())
                    .col(ColumnDef::new(RuleFirings::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .from(RuleFirings::Table, RuleFirings::EventId)
                            .to(Events::Table, Events::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(RuleFirings::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum RuleFirings {
    Table,
    Id,
    EventId,
    RuleId,
    Matched,
    Status,
    ActionResult,
    Error,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Events {
    Table,
    Id,
}
