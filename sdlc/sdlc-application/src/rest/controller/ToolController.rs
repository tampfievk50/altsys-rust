use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::ToolResponse::ToolResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::ToolPayloads::{CreateToolRequest, UpdateToolRequest};

#[utoipa::path(
    post,
    path = "/api/v1/tools",
    tag = "Tools",
    request_body = CreateToolRequest,
    responses(
        (status = 201, description = "Tool registered", body = ApiResponse<ToolResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_tool(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateToolRequest>,
) -> Result<Json<ApiResponse<ToolResponse>>, AppError> {
    let tool = state.tool_service.create_tool(payload.into()).await?;
    Ok(Json(ApiResponse::created(tool)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/tools",
    tag = "Tools",
    params(
        ("tenant_id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "List tools available to a tenant (tenant-specific + global)", body = ApiResponse<Vec<ToolResponse>>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_tools_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<ToolResponse>>>, AppError> {
    let tools = state.tool_service.find_tools_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(tools)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tools/{id}",
    tag = "Tools",
    params(
        ("id" = Uuid, Path, description = "Tool ID")
    ),
    responses(
        (status = 200, description = "Get tool by ID", body = ApiResponse<ToolResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn get_tool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ToolResponse>>, AppError> {
    let tool = state.tool_service.find_tool_by_id(id).await?;
    Ok(Json(ApiResponse::success(tool)))
}

#[utoipa::path(
    put,
    path = "/api/v1/tools/{id}",
    tag = "Tools",
    params(
        ("id" = Uuid, Path, description = "Tool ID")
    ),
    request_body = UpdateToolRequest,
    responses(
        (status = 200, description = "Tool updated", body = ApiResponse<ToolResponse>)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn update_tool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateToolRequest>,
) -> Result<Json<ApiResponse<ToolResponse>>, AppError> {
    let tool = state.tool_service.update_tool(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(tool)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/tools/{id}",
    tag = "Tools",
    params(
        ("id" = Uuid, Path, description = "Tool ID")
    ),
    responses(
        (status = 204, description = "Tool deleted")
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn delete_tool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.tool_service.delete_tool(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
