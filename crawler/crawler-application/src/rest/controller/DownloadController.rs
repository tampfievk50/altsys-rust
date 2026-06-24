use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use tracing::info;
use uuid::Uuid;

use crawler_domain::dto::DownloadVideoCommand::DownloadVideoCommand;
#[allow(unused_imports)]
use crawler_domain::dto::DownloadVideoResponse::DownloadVideoResponse;

use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::payload::DownloadVideoPayload::DownloadVideoPayload;
use crate::rest::response::ApiResponse::ApiResponse;

use crate::state::AppState::AppState;

#[utoipa::path(
    post,
    path = "/api/v1/downloads",
    tag = "Downloads",
    request_body = DownloadVideoPayload,
    responses(
        (status = 200, description = "Download task created", body = DownloadVideoResponse),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn create_download(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DownloadVideoPayload>,
) -> Result<impl IntoResponse, AppError> {
    info!(url = %payload.url, "Received download request");

    let command = DownloadVideoCommand { url: payload.url };

    let response = state
        .download_service
        .create_download(command)
        .await
        .map_err(AppError::from)?;

    Ok(Json(ApiResponse::success(response)))
}

#[utoipa::path(
    get,
    path = "/api/v1/downloads/{id}",
    tag = "Downloads",
    params(
        ("id" = Uuid, Path, description = "Download UUID")
    ),
    responses(
        (status = 200, description = "Download found", body = DownloadVideoResponse),
        (status = 404, description = "Download not found"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn get_download_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    info!(download_id = %id, "Fetching download by ID");

    let response = state
        .download_service
        .find_download_by_id(id)
        .await
        .map_err(AppError::from)?;

    Ok(Json(ApiResponse::success(response)))
}

#[utoipa::path(
    get,
    path = "/api/v1/downloads",
    tag = "Downloads",
    responses(
        (status = 200, description = "List of downloads", body = Vec<DownloadVideoResponse>),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn list_downloads(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    info!("Listing all downloads");

    let response = state
        .download_service
        .find_all_downloads()
        .await
        .map_err(AppError::from)?;

    Ok(Json(ApiResponse::success(response)))
}
