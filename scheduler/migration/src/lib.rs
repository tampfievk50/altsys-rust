pub use sea_orm_migration::prelude::*;

mod m20260723_000001_create_schedulers_table;
mod m20260723_000002_create_job_executions_table;
mod m20260723_000003_alter_schedulers_command_support;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260723_000001_create_schedulers_table::Migration),
            Box::new(m20260723_000002_create_job_executions_table::Migration),
            Box::new(m20260723_000003_alter_schedulers_command_support::Migration),
        ]
    }
}
