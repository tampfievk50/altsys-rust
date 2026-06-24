pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_tenants_table;
mod m20240101_000002_create_users_table;
mod m20240101_000003_create_roles_table;
mod m20240101_000004_create_permissions_table;
mod m20240101_000005_create_user_roles_table;
mod m20240101_000006_create_role_permissions_table;
mod m20240101_000007_create_refresh_tokens_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_tenants_table::Migration),
            Box::new(m20240101_000002_create_users_table::Migration),
            Box::new(m20240101_000003_create_roles_table::Migration),
            Box::new(m20240101_000004_create_permissions_table::Migration),
            Box::new(m20240101_000005_create_user_roles_table::Migration),
            Box::new(m20240101_000006_create_role_permissions_table::Migration),
            Box::new(m20240101_000007_create_refresh_tokens_table::Migration),
        ]
    }
}
