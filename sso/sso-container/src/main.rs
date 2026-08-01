use axum::Router;
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;
use migration::MigratorTrait;

use sso_application::config::AppConfig::create_app_state;
use sso_application::config::CasbinConfig::CasbinConfig;
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
    let enforcer = CasbinConfig::init_enforcer(&db).await;

    // Create AppState
    let state = create_app_state(db, enforcer).await;

    // Build the router with Swagger UI
    let mut app = create_router(state);

    // Add Swagger UI
    app = app.merge(
        SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );

    // Browser-based clients (e.g. the admin panel) hit this from a different
    // origin/port — without this, the browser blocks every request at the
    // CORS preflight before it ever reaches the handler.
    app = app.layer(CorsLayer::permissive());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    info!("Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
