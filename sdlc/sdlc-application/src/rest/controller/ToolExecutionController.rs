use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::ToolExecutionResult::ToolExecutionResult;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::ToolPayloads::ExecuteToolRequest;

#[utoipa::path(
    post,
    path = "/api/v1/tools/{id}/execute",
    tag = "Tool Execution",
    params(
        ("id" = Uuid, Path, description = "Tool ID")
    ),
    request_body = ExecuteToolRequest,
    responses(
        (status = 200, description = "Tool action executed (see `success` field for outcome)", body = ApiResponse<ToolExecutionResult>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn execute_tool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ExecuteToolRequest>,
) -> Result<Json<ApiResponse<ToolExecutionResult>>, AppError> {
    let result = state.tool_execution_service.execute_tool(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(result)))
}
