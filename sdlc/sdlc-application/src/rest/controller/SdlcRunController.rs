use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::SdlcRunResponse::SdlcRunResponse;
use sdlc_domain::dto::SdlcStepExecutionResponse::SdlcStepExecutionResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::SdlcRunPayloads::StartSdlcRunRequest;

#[utoipa::path(
    post,
    path = "/api/v1/sdlc-runs",
    tag = "Autonomous SDLC",
    request_body = StartSdlcRunRequest,
    responses(
        (status = 201, description = "Autonomous SDLC pipeline run to completion, failure, or an optional-step skip", body = ApiResponse<SdlcRunResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn start_run(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<StartSdlcRunRequest>,
) -> Result<Json<ApiResponse<SdlcRunResponse>>, AppError> {
    let run = state.sdlc_run_service.start_run(payload.into()).await?;
    Ok(Json(ApiResponse::created(run)))
}

#[utoipa::path(
    get,
    path = "/api/v1/sdlc-runs/{id}",
    tag = "Autonomous SDLC",
    params(
        ("id" = Uuid, Path, description = "SDLC run ID")
    ),
    responses(
        (status = 200, description = "Get an SDLC run by ID", body = ApiResponse<SdlcRunResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_run(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SdlcRunResponse>>, AppError> {
    let run = state.sdlc_run_service.find_run_by_id(id).await?;
    Ok(Json(ApiResponse::success(run)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/sdlc-runs",
    tag = "Autonomous SDLC",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List SDLC runs for a tenant", body = ApiResponse<Vec<SdlcRunResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_runs_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<SdlcRunResponse>>>, AppError> {
    let runs = state.sdlc_run_service.find_runs_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(runs)))
}

#[utoipa::path(
    get,
    path = "/api/v1/sdlc-runs/{id}/steps",
    tag = "Autonomous SDLC",
    params(
        ("id" = Uuid, Path, description = "SDLC run ID")
    ),
    responses(
        (status = 200, description = "Checkpoint log: every attempt of every pipeline step run so far", body = ApiResponse<Vec<SdlcStepExecutionResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_step_executions(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<SdlcStepExecutionResponse>>>, AppError> {
    let steps = state.sdlc_run_service.find_step_executions(id).await?;
    Ok(Json(ApiResponse::success(steps)))
}
