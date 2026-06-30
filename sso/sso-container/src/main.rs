use std::sync::Arc;
use axum::Router;
use casbin::{CoreApi, DefaultModel, Enforcer, Model};
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;
use migration::MigratorTrait;

use sso_application::config::AppConfig::create_app_state;
use sso_application::config::DatabaseConfig::DatabaseConfig;
use sso_application::rest::router::ApiRouter::create_router;
use sso_application::openapi::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    info!("Starting SSO Service...");

    // Connect to database
    let db = DatabaseConfig::connect().await.expect("Failed to connect to database");
    
    // Run migrations
    info!("Running database migrations...");
    migration::Migrator::up(&db, None).await.expect("Failed to run migrations");

    // Initialize Casbin Enforcer
    let enforcer = init_casbin_enforcer(&db).await;

    // Create AppState
    let state = create_app_state(db, enforcer).await;

    // Build the router with Swagger UI
    let mut app = create_router(state);
    
    // Add Swagger UI
    app = app.merge(
        SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );

    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    info!("Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn init_casbin_enforcer(db: &DatabaseConnection) -> Arc<RwLock<Enforcer>> {
    info!("Initializing Casbin enforcer from database...");
    
    let model_text = r#"
    [request_definition]
    r = sub, obj, act
    
    [policy_definition]
    p = sub, obj, act
    
    [role_definition]
    g = _, _
    
    [policy_effect]
    e = some(where (p.eft == allow))
    
    [matchers]
    m = g(r.sub, p.sub) && r.obj == p.obj && r.act == p.act || r.sub == "admin"
    "#;
    
    let model = DefaultModel::from_str(model_text).await.expect("Failed to load Casbin model");
    let adapter = sea_orm_adapter::SeaOrmAdapter::new(db.clone()).await.expect("Failed to create casbin sea-orm adapter");
    
    // Load policy from DB (SeaOrmAdapter will manage policies)
    // Removed unused repos as SeaOrmAdapter accesses DB directly for casbin tables.
    
    // Casbin's API requires us to add policies to the enforcer after creation
    let enforcer = Enforcer::new(model, adapter).await.expect("Failed to create enforcer");

    Arc::new(RwLock::new(enforcer))
}
