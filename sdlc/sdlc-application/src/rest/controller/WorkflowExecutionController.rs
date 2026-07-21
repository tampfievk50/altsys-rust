use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::ApprovalDecisionCommand::ApprovalDecisionCommand;
use sdlc_domain::dto::WorkflowExecutionResponse::WorkflowExecutionResponse;
use sdlc_domain::dto::WorkflowNodeExecutionResponse::WorkflowNodeExecutionResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::WorkflowExecutionPayloads::{ApprovalDecisionRequest, StartWorkflowExecutionRequest};

#[utoipa::path(
    post,
    path = "/api/v1/workflow-executions",
    tag = "Workflow Executions",
    request_body = StartWorkflowExecutionRequest,
    responses(
        (status = 201, description = "Workflow execution started and run to completion, failure, or an approval gate", body = ApiResponse<WorkflowExecutionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn start_execution(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<StartWorkflowExecutionRequest>,
) -> Result<Json<ApiResponse<WorkflowExecutionResponse>>, AppError> {
    let execution = state.execution_service.start_execution(payload.into()).await?;
    Ok(Json(ApiResponse::created(execution)))
}

#[utoipa::path(
    get,
    path = "/api/v1/workflow-executions/{id}",
    tag = "Workflow Executions",
    params(
        ("id" = Uuid, Path, description = "Workflow execution ID")
    ),
    responses(
        (status = 200, description = "Get a workflow execution by ID", body = ApiResponse<WorkflowExecutionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_execution(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<WorkflowExecutionResponse>>, AppError> {
    let execution = state.execution_service.find_execution_by_id(id).await?;
    Ok(Json(ApiResponse::success(execution)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/workflow-executions",
    tag = "Workflow Executions",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List workflow executions for a tenant", body = ApiResponse<Vec<WorkflowExecutionResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_executions_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<WorkflowExecutionResponse>>>, AppError> {
    let executions = state.execution_service.find_executions_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(executions)))
}

#[utoipa::path(
    get,
    path = "/api/v1/workflow-executions/{id}/node-executions",
    tag = "Workflow Executions",
    params(
        ("id" = Uuid, Path, description = "Workflow execution ID")
    ),
    responses(
        (status = 200, description = "Checkpoint log: every attempt of every node run so far", body = ApiResponse<Vec<WorkflowNodeExecutionResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_node_executions(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<WorkflowNodeExecutionResponse>>>, AppError> {
    let node_executions = state.execution_service.find_node_executions(id).await?;
    Ok(Json(ApiResponse::success(node_executions)))
}

#[utoipa::path(
    post,
    path = "/api/v1/workflow-executions/{id}/nodes/{node_id}/decide",
    tag = "Workflow Executions",
    params(
        ("id" = Uuid, Path, description = "Workflow execution ID"),
        ("node_id" = String, Path, description = "Approval node ID")
    ),
    request_body = ApprovalDecisionRequest,
    responses(
        (status = 200, description = "Approval decision recorded; execution resumed from checkpoint state", body = ApiResponse<WorkflowExecutionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn decide_approval(
    State(state): State<Arc<AppState>>,
    Path((id, node_id)): Path<(Uuid, String)>,
    Json(payload): Json<ApprovalDecisionRequest>,
) -> Result<Json<ApiResponse<WorkflowExecutionResponse>>, AppError> {
    let command: ApprovalDecisionCommand = payload.into();
    let execution = state.execution_service.decide_approval(id, &node_id, command).await?;
    Ok(Json(ApiResponse::success(execution)))
}
