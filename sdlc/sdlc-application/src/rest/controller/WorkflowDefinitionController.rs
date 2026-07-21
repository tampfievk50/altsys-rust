use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::WorkflowDefinitionResponse::WorkflowDefinitionResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::WorkflowDefinitionPayloads::{CreateWorkflowDefinitionRequest, UpdateWorkflowDefinitionRequest};

#[utoipa::path(
    post,
    path = "/api/v1/workflow-definitions",
    tag = "Workflow Definitions",
    request_body = CreateWorkflowDefinitionRequest,
    responses(
        (status = 201, description = "Workflow definition version created", body = ApiResponse<WorkflowDefinitionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_workflow_definition(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateWorkflowDefinitionRequest>,
) -> Result<Json<ApiResponse<WorkflowDefinitionResponse>>, AppError> {
    let definition = state.definition_service.create_workflow_definition(payload.into()).await?;
    Ok(Json(ApiResponse::created(definition)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/workflow-definitions",
    tag = "Workflow Definitions",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List the latest version of every workflow definition key for a tenant", body = ApiResponse<Vec<WorkflowDefinitionResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_workflow_definitions_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<WorkflowDefinitionResponse>>>, AppError> {
    let definitions = state.definition_service.find_workflow_definitions_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(definitions)))
}

#[utoipa::path(
    get,
    path = "/api/v1/workflow-definitions/{id}",
    tag = "Workflow Definitions",
    params(
        ("id" = Uuid, Path, description = "Workflow definition ID")
    ),
    responses(
        (status = 200, description = "Get a workflow definition version by ID", body = ApiResponse<WorkflowDefinitionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_workflow_definition(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<WorkflowDefinitionResponse>>, AppError> {
    let definition = state.definition_service.find_workflow_definition_by_id(id).await?;
    Ok(Json(ApiResponse::success(definition)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/workflow-definitions/{key}/latest",
    tag = "Workflow Definitions",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID"),
        ("key" = String, Path, description = "Workflow definition key")
    ),
    responses(
        (status = 200, description = "Get the latest version for a key", body = ApiResponse<WorkflowDefinitionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_latest_workflow_definition_by_key(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, key)): Path<(Uuid, String)>,
) -> Result<Json<ApiResponse<WorkflowDefinitionResponse>>, AppError> {
    let definition = state.definition_service.find_latest_workflow_definition_by_key(tenant_id, &key).await?;
    Ok(Json(ApiResponse::success(definition)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/workflow-definitions/{key}/versions",
    tag = "Workflow Definitions",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID"),
        ("key" = String, Path, description = "Workflow definition key")
    ),
    responses(
        (status = 200, description = "List all versions for a key, oldest first", body = ApiResponse<Vec<WorkflowDefinitionResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_workflow_definition_versions_by_key(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, key)): Path<(Uuid, String)>,
) -> Result<Json<ApiResponse<Vec<WorkflowDefinitionResponse>>>, AppError> {
    let versions = state.definition_service.find_workflow_definition_versions_by_key(tenant_id, &key).await?;
    Ok(Json(ApiResponse::success(versions)))
}

#[utoipa::path(
    put,
    path = "/api/v1/workflow-definitions/{id}",
    tag = "Workflow Definitions",
    params(
        ("id" = Uuid, Path, description = "Workflow definition ID")
    ),
    request_body = UpdateWorkflowDefinitionRequest,
    responses(
        (status = 200, description = "Workflow definition updated in place", body = ApiResponse<WorkflowDefinitionResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_workflow_definition(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateWorkflowDefinitionRequest>,
) -> Result<Json<ApiResponse<WorkflowDefinitionResponse>>, AppError> {
    let definition = state.definition_service.update_workflow_definition(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(definition)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/workflow-definitions/{id}",
    tag = "Workflow Definitions",
    params(
        ("id" = Uuid, Path, description = "Workflow definition ID")
    ),
    responses(
        (status = 204, description = "Workflow definition version deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_workflow_definition(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.definition_service.delete_workflow_definition(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
