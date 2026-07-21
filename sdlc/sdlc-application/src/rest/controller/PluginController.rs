use std::sync::Arc;
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use sdlc_domain::dto::PluginResponse::PluginResponse;
use crate::state::AppState::AppState;
use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::rest::payload::PluginPayloads::{CreatePluginRequest, UpdatePluginRequest};

#[utoipa::path(
    post,
    path = "/api/v1/plugins",
    tag = "Plugins",
    request_body = CreatePluginRequest,
    responses(
        (status = 201, description = "Plugin registered", body = ApiResponse<PluginResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn create_plugin(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePluginRequest>,
) -> Result<Json<ApiResponse<PluginResponse>>, AppError> {
    let plugin = state.plugin_service.create_plugin(payload.into()).await?;
    Ok(Json(ApiResponse::created(plugin)))
}

#[utoipa::path(
    get,
    path = "/api/v1/tenants/{tenant_id}/plugins",
    tag = "Plugins",
    params(("tenant_id" = Uuid, Path, description = "Tenant ID")),
    responses(
        (status = 200, description = "List plugins available to a tenant (tenant-specific + global)", body = ApiResponse<Vec<PluginResponse>>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_plugins_by_tenant(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<PluginResponse>>>, AppError> {
    let plugins = state.plugin_service.find_plugins_by_tenant(tenant_id).await?;
    Ok(Json(ApiResponse::success(plugins)))
}

#[utoipa::path(
    get,
    path = "/api/v1/plugins/{id}",
    tag = "Plugins",
    params(("id" = Uuid, Path, description = "Plugin ID")),
    responses(
        (status = 200, description = "Get plugin by ID", body = ApiResponse<PluginResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_plugin(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<PluginResponse>>, AppError> {
    let plugin = state.plugin_service.find_plugin_by_id(id).await?;
    Ok(Json(ApiResponse::success(plugin)))
}

#[utoipa::path(
    put,
    path = "/api/v1/plugins/{id}",
    tag = "Plugins",
    params(("id" = Uuid, Path, description = "Plugin ID")),
    request_body = UpdatePluginRequest,
    responses(
        (status = 200, description = "Plugin updated", body = ApiResponse<PluginResponse>)
    ),
    security(("bearerAuth" = []))
)]
pub async fn update_plugin(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePluginRequest>,
) -> Result<Json<ApiResponse<PluginResponse>>, AppError> {
    let plugin = state.plugin_service.update_plugin(id, payload.into()).await?;
    Ok(Json(ApiResponse::success(plugin)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/plugins/{id}",
    tag = "Plugins",
    params(("id" = Uuid, Path, description = "Plugin ID")),
    responses(
        (status = 204, description = "Plugin deleted")
    ),
    security(("bearerAuth" = []))
)]
pub async fn delete_plugin(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    state.plugin_service.delete_plugin(id).await?;
    Ok(Json(ApiResponse::no_content()))
}
