use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::InstantiateTemplateResponse::InstantiateTemplateResponse;
use sdlc_domain::dto::WorkflowTemplateResponse::WorkflowTemplateResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::WorkflowTemplatePayloads::{CreateWorkflowTemplateRequest, InstantiateTemplateRequest, UpdateWorkflowTemplateRequest};

#[utoipa::path(
    post,
    path = "/api/v1/workflow-templates",
    tag = "Workflow Templates",
    request_body = CreateWorkflowTemplateRequest,
    responses(
        (status = 201, description = "Workflow template version created", body = ApiResponse<WorkflowTemplateResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn create_template(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateWorkflowTemplateRequest>,
) -> Result<Json<ApiResponse<WorkflowTemplateResponse>>, AppError> {
    let template = state.template_service.create_template(payload.into()).await?;
    Ok(Json(ApiResponse::created(template)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/workflow-templates",
    tag = "Workflow Templates",
    params(("tenant_id" = Uuid, Path, description = "Tenant ID")),
    responses(
        (status = 200, description = "List the latest version of every workflow template key for a tenant", body = ApiResponse<Vec<WorkflowTemplateResponse>>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_templates_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<WorkflowTemplateResponse>>>, AppError> {
    let templates = state.template_service.find_templates_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(templates)))
}

#[utoipa::path(
    get,
    path = "/api/v1/workflow-templates/{id}",
    tag = "Workflow Templates",
    params(("id" = Uuid, Path, description = "Workflow template ID")),
    responses(
        (status = 200, description = "Get a workflow template version by ID", body = ApiResponse<WorkflowTemplateResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_template(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<WorkflowTemplateResponse>>, AppError> {
    let template = state.template_service.find_template_by_id(id).await?;
    Ok(Json(ApiResponse::success(template)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/workflow-templates/{key}/latest",
    tag = "Workflow Templates",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID"),
        ("key" = String, Path, description = "Workflow template key")
    ),
    responses(
        (status = 200, description = "Get the latest version for a key", body = ApiResponse<WorkflowTemplateResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_latest_template_by_key(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, key)): Path<(Uuid, String)>,
) -> Result<Json<ApiResponse<WorkflowTemplateResponse>>, AppError> {
    let template = state.template_service.find_latest_template_by_key(tenant_id, &key).await?;
    Ok(Json(ApiResponse::success(template)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/workflow-templates/{key}/versions",
    tag = "Workflow Templates",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID"),
        ("key" = String, Path, description = "Workflow template key")
    ),
    responses(
        (status = 200, description = "List all versions for a key, oldest first", body = ApiResponse<Vec<WorkflowTemplateResponse>>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_template_versions_by_key(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, key)): Path<(Uuid, String)>,
) -> Result<Json<ApiResponse<Vec<WorkflowTemplateResponse>>>, AppError> {
    let versions = state.template_service.find_template_versions_by_key(tenant_id, &key).await?;
    Ok(Json(ApiResponse::success(versions)))
}

#[utoipa::path(
    put,
    path = "/api/v1/workflow-templates/{id}",
    tag = "Workflow Templates",
    params(("id" = Uuid, Path, description = "Workflow template ID")),
    request_body = UpdateWorkflowTemplateRequest,
    responses(
        (status = 200, description = "Workflow template updated in place", body = ApiResponse<WorkflowTemplateResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn update_template(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateWorkflowTemplateRequest>,
) -> Result<Json<ApiResponse<WorkflowTemplateResponse>>, AppError> {
    let template = state.template_service.update_template(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(template)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/workflow-templates/{id}",
    tag = "Workflow Templates",
    params(("id" = Uuid, Path, description = "Workflow template ID")),
    responses(
        (status = 204, description = "Workflow template version deleted")
    ),
    security(("bearerAuth" = []))
)]
pub async fn delete_template(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.template_service.delete_template(id).await?;
    Ok(Json(ApiResponse::no_content()))
}

#[utoipa::path(
    post,
    path = "/api/v1/workflow-templates/{id}/instantiate",
    tag = "Workflow Templates",
    params(("id" = Uuid, Path, description = "Workflow template ID")),
    request_body = InstantiateTemplateRequest,
    responses(
        (status = 200, description = "Resolved WorkflowGraph JSON, ready to submit to the Workflow service", body = ApiResponse<InstantiateTemplateResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn instantiate_template(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<InstantiateTemplateRequest>,
) -> Result<Json<ApiResponse<InstantiateTemplateResponse>>, AppError> {
    let result = state.template_service.instantiate_template(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(result)))
}
