use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    if std::env::var("DATABASE_URL").is_err() {
        let host = std::env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
        let user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set");
        let pass = std::env::var("DATABASE_PASS").expect("DATABASE_PASS must be set");
        let name = std::env::var("DATABASE_NAME").unwrap_or_else(|_| "rustcms".to_string());
        let url = format!("postgres://{}:{}@{}/{}", user, pass, host, name);
        std::env::set_var("DATABASE_URL", url);
    }

    cli::run_cli(migration::Migrator).await;
}
