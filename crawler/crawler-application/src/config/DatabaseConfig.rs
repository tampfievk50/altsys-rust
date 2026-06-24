use sea_orm::{Database, DatabaseConnection, DbErr};

pub struct DatabaseConfig;

impl DatabaseConfig {
    pub async fn connect() -> Result<DatabaseConnection, DbErr> {
        let host = std::env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
        let user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set");
        let pass = std::env::var("DATABASE_PASS").expect("DATABASE_PASS must be set");
        let name = std::env::var("DATABASE_NAME").unwrap_or_else(|_| "rustcms".to_string());
        
        let database_url = format!("postgres://{}:{}@{}/{}", user, pass, host, name);

        Database::connect(&database_url).await
    }
}
