#![allow(non_snake_case)]

use std::env;
use std::net::SocketAddr;

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crawler_application::rest::router::DownloadRouter::DownloadRouter;
use crawler_application::rest::router::UserRouter::UserRouter;
use crawler_application::config::{AppConfig, DatabaseConfig};
use crawler_application::openapi::ApiDoc;
use migration::{Migrator, MigratorTrait};

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let download_dir = env::var("DOWNLOAD_DIR").unwrap_or_else(|_| "./downloads".to_string());
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("SERVER_PORT must be a valid port number");

    tokio::fs::create_dir_all(&download_dir)
        .await
        .expect("Failed to create download directory");

    let db = DatabaseConfig::DatabaseConfig::connect()
        .await
        .expect("Failed to connect to Postgres database");

    Migrator::up(&db, None).await.expect("Failed to run migrations");

    info!("Database initialized successfully");

    let app_state = AppConfig::create_app_state(db, download_dir);

    let app = DownloadRouter::routes()
        .merge(UserRouter::routes())
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(app_state)
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        );

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid server address");

    info!("🚀 Crawler API server starting on {}", addr);
    info!("📖 Swagger UI available at http://{}:{}/swagger-ui", host, port);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
