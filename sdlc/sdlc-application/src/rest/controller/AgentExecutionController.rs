use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::AgentExecutionResponse::AgentExecutionResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::AgentExecutionPayloads::ExecuteAgentRequest;

#[utoipa::path(
    post,
    path = "/api/v1/agents/{id}/execute",
    tag = "Agent Execution",
    params(
        ("id" = Uuid, Path, description = "Agent ID")
    ),
    request_body = ExecuteAgentRequest,
    responses(
        (status = 201, description = "Agent executed (see `status` field for outcome)", body = ApiResponse<AgentExecutionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn execute_agent(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ExecuteAgentRequest>,
) -> Result<Json<ApiResponse<AgentExecutionResponse>>, AppError> {
    let execution = state.agent_execution_service.execute_agent(id, payload.into()).await?;
    Ok(Json(ApiResponse::created(execution)))
}

#[utoipa::path(
    get,
    path = "/api/v1/agent-executions/{id}",
    tag = "Agent Execution",
    params(
        ("id" = Uuid, Path, description = "Agent execution ID")
    ),
    responses(
        (status = 200, description = "Get an agent execution by ID", body = ApiResponse<AgentExecutionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_execution(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AgentExecutionResponse>>, AppError> {
    let execution = state.agent_execution_service.find_execution_by_id(id).await?;
    Ok(Json(ApiResponse::success(execution)))
}

#[utoipa::path(
    get,
    path = "/api/v1/agents/{id}/executions",
    tag = "Agent Execution",
    params(
        ("id" = Uuid, Path, description = "Agent ID")
    ),
    responses(
        (status = 200, description = "List executions for an agent", body = ApiResponse<Vec<AgentExecutionResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_executions_by_agent(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<AgentExecutionResponse>>>, AppError> {
    let executions = state.agent_execution_service.find_executions_by_agent(id).await?;
    Ok(Json(ApiResponse::success(executions)))
}
