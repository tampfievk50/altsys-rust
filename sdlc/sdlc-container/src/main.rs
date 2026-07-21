use dotenvy::dotenv;
use migration::MigratorTrait;
use tower_http::cors::CorsLayer;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

use sdlc_application::config::AppConfig::create_app_state;
use sdlc_application::config::DatabaseConfig::DatabaseConfig;
use sdlc_application::openapi::ApiDoc;
use sdlc_application::rest::router::ApiRouter::create_router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    info!("Starting SDLC Platform Service (merged platform+tools+knowledge+workflow+agents+sdlc+automation)...");

    let db = DatabaseConfig::connect().await.expect("Failed to connect to database");

    info!("Running database migrations...");
    migration::Migrator::up(&db, None).await.expect("Failed to run migrations");

    let state = create_app_state(db).await;

    let mut app = create_router(state);

    app = app.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // Browser-based clients (e.g. the admin panel) hit this from a different
    // origin/port.
    app = app.layer(CorsLayer::permissive());

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3010".to_string());
    let addr = format!("{}:{}", host, port);

    info!("Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
