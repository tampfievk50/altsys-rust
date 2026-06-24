use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add audit columns to downloads table
        manager
            .alter_table(
                Table::alter()
                    .table(Downloads::Table)
                    .add_column(ColumnDef::new(Downloads::CreatedBy).uuid().null())
                    .add_column(ColumnDef::new(Downloads::UpdatedBy).uuid().null())
                    .to_owned(),
            )
            .await?;

        // Add audit columns to users table
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Users::CreatedBy).uuid().null())
                    .add_column(ColumnDef::new(Users::UpdatedBy).uuid().null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Downloads::Table)
                    .drop_column(Downloads::CreatedBy)
                    .drop_column(Downloads::UpdatedBy)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::CreatedBy)
                    .drop_column(Users::UpdatedBy)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Downloads {
    Table,
    CreatedBy,
    UpdatedBy,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    CreatedBy,
    UpdatedBy,
}
