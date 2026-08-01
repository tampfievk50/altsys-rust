use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
            ALTER TABLE schedulers
                ALTER COLUMN webhook_url DROP NOT NULL,
                ALTER COLUMN http_method DROP NOT NULL,
                ADD COLUMN job_type VARCHAR(20) NOT NULL DEFAULT 'WEBHOOK',
                ADD COLUMN command_line TEXT,
                ADD COLUMN working_dir TEXT,
                ADD COLUMN detached BOOLEAN NOT NULL DEFAULT FALSE,
                ADD COLUMN last_pid INTEGER;

            ALTER TABLE schedulers
                ADD CONSTRAINT chk_scheduler_job_type CHECK (
                    (job_type = 'WEBHOOK' AND webhook_url IS NOT NULL AND http_method IS NOT NULL)
                    OR
                    (job_type = 'COMMAND' AND command_line IS NOT NULL)
                );
            "#,
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
            ALTER TABLE schedulers DROP CONSTRAINT IF EXISTS chk_scheduler_job_type;
            ALTER TABLE schedulers
                DROP COLUMN IF EXISTS job_type,
                DROP COLUMN IF EXISTS command_line,
                DROP COLUMN IF EXISTS working_dir,
                DROP COLUMN IF EXISTS detached,
                DROP COLUMN IF EXISTS last_pid,
                ALTER COLUMN webhook_url SET NOT NULL,
                ALTER COLUMN http_method SET NOT NULL;
            "#,
        )
        .await?;
        Ok(())
    }
}
