use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use scheduler_domain::dto::ExecutionResponse::ExecutionResponse;
use scheduler_domain::dto::SchedulerResponse::SchedulerResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::SchedulerPayloads::{CreateSchedulerRequest, UpdateSchedulerRequest};

#[utoipa::path(
    post,
    path = "/api/v1/schedulers",
    tag = "Schedulers",
    request_body = CreateSchedulerRequest,
    responses(
        (status = 201, description = "Scheduler created", body = ApiResponse<SchedulerResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_scheduler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSchedulerRequest>,
) -> Result<Json<ApiResponse<SchedulerResponse>>, AppError> {
    let scheduler = state.scheduler_service.create_scheduler(payload.into()).await?;
    Ok(Json(ApiResponse::created(scheduler)))
}

#[utoipa::path(
    get,
    path = "/api/v1/schedulers",
    tag = "Schedulers",
    responses(
        (status = 200, description = "List all schedulers", body = ApiResponse<Vec<SchedulerResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_all_schedulers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<SchedulerResponse>>>, AppError> {
    let schedulers = state.scheduler_service.find_all_schedulers().await?;
    Ok(Json(ApiResponse::success(schedulers)))
}

#[utoipa::path(
    get,
    path = "/api/v1/schedulers/{id}",
    tag = "Schedulers",
    params(
        ("id" = Uuid, Path, description = "Scheduler ID")
    ),
    responses(
        (status = 200, description = "Get scheduler by ID", body = ApiResponse<SchedulerResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_scheduler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SchedulerResponse>>, AppError> {
    let scheduler = state.scheduler_service.find_scheduler_by_id(id).await?;
    Ok(Json(ApiResponse::success(scheduler)))
}

#[utoipa::path(
    put,
    path = "/api/v1/schedulers/{id}",
    tag = "Schedulers",
    params(
        ("id" = Uuid, Path, description = "Scheduler ID")
    ),
    request_body = UpdateSchedulerRequest,
    responses(
        (status = 200, description = "Scheduler updated", body = ApiResponse<SchedulerResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_scheduler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateSchedulerRequest>,
) -> Result<Json<ApiResponse<SchedulerResponse>>, AppError> {
    let scheduler = state.scheduler_service.update_scheduler(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(scheduler)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/schedulers/{id}",
    tag = "Schedulers",
    params(
        ("id" = Uuid, Path, description = "Scheduler ID")
    ),
    responses(
        (status = 204, description = "Scheduler deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_scheduler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.scheduler_service.delete_scheduler(id).await?;
    Ok(Json(ApiResponse::no_content()))
}

#[utoipa::path(
    post,
    path = "/api/v1/schedulers/{id}/pause",
    tag = "Schedulers",
    params(
        ("id" = Uuid, Path, description = "Scheduler ID")
    ),
    responses(
        (status = 200, description = "Scheduler paused", body = ApiResponse<SchedulerResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn pause_scheduler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SchedulerResponse>>, AppError> {
    let scheduler = state.scheduler_service.pause_scheduler(id).await?;
    Ok(Json(ApiResponse::success(scheduler)))
}

#[utoipa::path(
    post,
    path = "/api/v1/schedulers/{id}/resume",
    tag = "Schedulers",
    params(
        ("id" = Uuid, Path, description = "Scheduler ID")
    ),
    responses(
        (status = 200, description = "Scheduler resumed", body = ApiResponse<SchedulerResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn resume_scheduler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SchedulerResponse>>, AppError> {
    let scheduler = state.scheduler_service.resume_scheduler(id).await?;
    Ok(Json(ApiResponse::success(scheduler)))
}

#[utoipa::path(
    post,
    path = "/api/v1/schedulers/{id}/run",
    tag = "Schedulers",
    params(
        ("id" = Uuid, Path, description = "Scheduler ID")
    ),
    responses(
        (status = 200, description = "Scheduler run triggered immediately", body = ApiResponse<ExecutionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn run_scheduler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ExecutionResponse>>, AppError> {
    let execution = state.scheduler_runner.run_scheduler(id).await?;
    Ok(Json(ApiResponse::success(execution)))
}
