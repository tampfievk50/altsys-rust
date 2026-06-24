use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use crate::rest::controller::DownloadController::{
    create_download, get_download_by_id, list_downloads,
};
use crate::state::AppState::AppState;

pub struct DownloadRouter;

impl DownloadRouter {
    pub fn routes() -> Router<Arc<AppState>> {
        Router::new()
            .route("/api/v1/downloads", post(create_download))
            .route("/api/v1/downloads", get(list_downloads))
            .route("/api/v1/downloads/{id}", get(get_download_by_id))
    }
}
