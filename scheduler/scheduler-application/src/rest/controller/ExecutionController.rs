use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use scheduler_domain::dto::ExecutionResponse::ExecutionResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;

#[utoipa::path(
    get,
    path = "/api/v1/schedulers/{id}/executions",
    tag = "Executions",
    params(
        ("id" = Uuid, Path, description = "Scheduler ID")
    ),
    responses(
        (status = 200, description = "Execution history for a scheduler", body = ApiResponse<Vec<ExecutionResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_executions_by_scheduler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<ExecutionResponse>>>, AppError> {
    let executions = state.execution_service.find_executions_by_scheduler(id).await?;
    Ok(Json(ApiResponse::success(executions)))
}

#[utoipa::path(
    get,
    path = "/api/v1/executions/{id}",
    tag = "Executions",
    params(
        ("id" = Uuid, Path, description = "Execution ID")
    ),
    responses(
        (status = 200, description = "Get execution by ID", body = ApiResponse<ExecutionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_execution(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ExecutionResponse>>, AppError> {
    let execution = state.execution_service.find_execution_by_id(id).await?;
    Ok(Json(ApiResponse::success(execution)))
}
